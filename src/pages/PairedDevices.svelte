<script lang="ts">
import { onMount } from 'svelte'
import { invoke } from '@tauri-apps/api/tauri'
import { listen, type Event as TauriEvent } from '@tauri-apps/api/event'

// Utils
import clickOutside from '../utils/clickOutside'

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
let isScanListOpen = false
let scannedDevices = []
let devices: Array<Device> = [
  {
    type: DeviceType.HeartRate,
    title: 'Heart Rate Monitor',
    isConnected: false,
  },
  {
    type: DeviceType.SmartTrainer,
    title: 'Smart trainer (Power Meter, Speed, and Cadence)',
    isConnected: false,
  },
]

onMount(async () => {
  isScanning = true
  await invoke('start_scan')
  await invoke('get_connected_devices')
})

listen('devices-connected', async (event: TauriEvent<any>) => {
  const { payload } = event

  const [hrm, sc] = devices

  if (hrm.isConnected && sc.isConnected) {
    await invoke('stop_scan')

    return
  }

  payload.forEach((connectedDevice: [string, string]) => {
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
})

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

  // Start scanning
  isScanListOpen = true

  if (!isScanning) {
    await invoke('start_scan')

    isScanning = true
  }
}

async function handleConnect(device: { id: string }) {
  await invoke('connect_device', { deviceId: device.id })

  // TODO: Implement connection here

  cleanStates()
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
  isScanListOpen = false
  scannedDevices = []
}
</script>

<div class="component-paired-devices">
  <h1 class="page-title">Paired Devices</h1>

  <div class="devices-list">
    {#each devices as device}
      <div class="device {device.isConnected ? 'is-connected' : ''}">
        <h3 class="title">{device.title}</h3>
        <div class="name">
          Name: {device.bleDevice?.name || '--'}
        </div>
        <div class="actions">
          <button
            class="{device.isConnected ? 'is-connected' : ''}"
            on:click="{() => handleAction(device)}"
            >{device.isConnected ? 'Disconnect' : 'Connect'}</button
          >
        </div>
      </div>
    {/each}
  </div>

  {#if isScanListOpen}
    <div class="scanned-devices-list" use:clickOutside>
      <h3 class="title">Scanning...</h3>
      <div class="list-container">
        {#each scannedDevices as device}
          <div class="device" on:click="{() => handleConnect(device)}">
            {device.name}
          </div>
        {/each}
      </div>
      <button on:click="{() => handleCloseScan()}" class="close-btn"
        >Close</button
      >
    </div>
  {/if}
</div>
