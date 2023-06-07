<script lang="ts">
import HeartIcon from 'svelte-icons/fa/FaHeartbeat.svelte'
import BikeIcon from 'svelte-icons/md/MdDirectionsBike.svelte'
import { onMount } from 'svelte'
import { invoke } from '@tauri-apps/api/tauri'
import { listen, type Event as TauriEvent } from '@tauri-apps/api/event'

// Utils
import clickOutside from '../../utils/clickOutside'

import './styles.css'

// Types and enums
enum DeviceType {
  HeartRate = 'hr',
  SmartTrainer = 'st',
}

interface Device {
  type: DeviceType
  title: string
  name?: string
  bleDevice?: {
    id: string
    name: string
  }
  isConnected: boolean
}

// States
let isScanning = false
let isConnecting = false
let scannedDevices = []
let devices: Array<Device> = [
  {
    type: DeviceType.HeartRate,
    title: 'Heart Rate',
    isConnected: false,
  },
  {
    type: DeviceType.SmartTrainer,
    title: 'Smart trainer',
    isConnected: false,
  },
]

listen('device-discovered', (event: TauriEvent<any>) => {
  const { payload } = event

  const [id, name] = payload

  const existing = scannedDevices.find((device) => device.id == id)

  if (existing) {
    return
  }

  scannedDevices = [
    ...scannedDevices,
    {
      id,
      name,
    },
  ]
})

async function handleAction(device: Device) {
  if (device.isConnected) {
    disconnectDevice(device)

    return
  }

  await invoke('start_scan')

  isScanning = true
}

async function handleConnect(device: { id: string }) {
  if (isConnecting) {
    return
  }

  isConnecting = true

  scannedDevices = scannedDevices.map((scannedDevice) => {
    if (scannedDevice.id == device.id) {
      return {
        ...scannedDevice,
        isConnecting: true,
      }
    }

    return scannedDevice
  })

  await invoke('connect_device', { deviceId: device.id })
  const connectedDevices: Array<[string, string]> = await invoke(
    'get_connected_devices'
  )

  connectedDevices.forEach((connectedDevice: [string, string]) => {
    const [id, name] = connectedDevice

    // TODO: Use appropriate identifiers
    if (name == 'Venu 2') {
      devices = devices.map((device) => {
        if (device.type == DeviceType.HeartRate) {
          return {
            ...device,
            bleDevice: {
              id,
              name,
            },
            isConnected: true,
          }
        }

        return device
      })
    }
  })

  handleCloseScan()
}

async function disconnectDevice(device: Device) {
  await invoke('disconnect_device', { deviceId: device.bleDevice.id })

  changeConnectionState(device.type, false)
}

async function changeConnectionState(type: DeviceType, isConnected: boolean) {
  devices = devices.map((device) => {
    if (device.type == type) {
      return {
        ...device,
        isConnected,
        bleDevice: isConnected ? device.bleDevice : null,
      }
    }

    return device
  })
}

async function handleCloseScan() {
  await invoke('stop_scan')

  cleanStates()
}

async function cleanStates() {
  isScanning = false
  isConnecting = false
  scannedDevices = []
}
</script>

<div class="component-paired-devices p-10">
  <div class="page-title text-xl font-bold text-center">Paired Devices</div>

  <div class="devices-list flex space-x-6 justify-center m-10">
    {#each devices as device}
      <button
        class="device relative box-border border p-4 rounded-2xl flex flex-col justify-between {device.isConnected
          ? 'is-connected'
          : ''}"
        on:click="{() => handleAction(device)}"
      >
        <div class="icon h-10 w-10 m-2">
          {#if device.type === DeviceType.HeartRate}
            <HeartIcon />
          {/if}
          {#if device.type === DeviceType.SmartTrainer}
            <BikeIcon />
          {/if}
        </div>

        {#if device.isConnected}
          <span class="connected-indicator absolute left-4 flex h-2 w-2">
            <span
              class="animate-ping absolute inline-flex h-full w-full rounded-full bg-primary-300 opacity-75"
            ></span>
            <span
              class="relative inline-flex rounded-full h-2 w-2 bg-primary-400"
            ></span>
          </span>
        {/if}

        <div class="name text-left">
          <div class="title font-bold flex space-x-2">
            <div>{device.title}</div>
          </div>
          <div class="device-name font-semibold text-secondary-100 text-xs">
            {device.isConnected
              ? `${device.bleDevice?.name || ''} connected`
              : 'Click to pair'}
          </div>
        </div>
      </button>
    {/each}
  </div>

  {#if isScanning}
    <div class="scanned-devices-list overflow-hidden" use:clickOutside>
      <div class="animate-pulse title">Scanning</div>
      <div class="list-container">
        {#each scannedDevices as device}
          <button
            class="device-item {device.isConnecting ? 'text-primary-400' : ''}"
            on:click="{() => handleConnect(device)}"
          >
            {device.name}
            {#if device.isConnecting}
              <div class="text-sm text-secondary-100 animate-pulse">
                Connecting...
              </div>
            {/if}
          </button>
        {/each}
      </div>
      <button on:click="{() => handleCloseScan()}" class="btn close-btn"
        >Close</button
      >
    </div>
  {/if}
</div>
