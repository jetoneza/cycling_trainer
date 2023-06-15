<script lang="ts">
// Libraries
import HeartIcon from 'svelte-icons/fa/FaHeartbeat.svelte'
import BikeIcon from 'svelte-icons/md/MdDirectionsBike.svelte'
import { invoke } from '@tauri-apps/api/tauri'
import { listen, type Event as TauriEvent } from '@tauri-apps/api/event'

// Components
import ScanList from './components/ScanList.svelte'

// Stores
import { devices, updateDevices } from '../../stores/devices'

// Utils
import clickOutside from '../../utils/clickOutside'

// Types
import { DeviceType, type Device } from '../../types'

// Styles
import './styles.css'

// States
let isScanning = false
let isConnecting = false
let scannedDevices = []

listen('device-discovered', (event: TauriEvent<any>) => {
  const { payload } = event

  const { id, local_name: name } = payload

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

  await invoke('start_scan', { scanFilter: device.type })

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
  const connectedDevices: Array<[string, string, string]> = await invoke(
    'get_connected_devices'
  )

  connectedDevices.forEach((connectedDevice: [string, string, string]) => {
    const [id, name, type] = connectedDevice

    updateDevices((device) => {
      if (device.type == type) {
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
  })

  handleCloseScan()
}

async function disconnectDevice(device: Device) {
  await invoke('disconnect_device', { deviceId: device.bleDevice.id })

  changeConnectionState(device.type, false)
}

async function changeConnectionState(type: DeviceType, isConnected: boolean) {
  updateDevices((device) => {
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

function getDeviceHRM(device: Device) {
  const { bpm, is_sensor_contact_supported, is_sensor_in_contact } =
    device.bleDevice.data

  if (!is_sensor_contact_supported) {
    return bpm
  }

  return is_sensor_in_contact ? bpm : '--'
}
</script>

<div class="component-paired-devices p-10">
  <div class="page-title text-xl font-bold text-center">Paired Devices</div>

  <div class="devices-list flex space-x-6 justify-center m-10">
    {#each $devices as device}
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
          {#if device.isConnected && device.type === DeviceType.HeartRate && device.bleDevice.data}
            <div class="text-white font-bold">
              <span class="text-5xl">{getDeviceHRM(device)}</span>
              <span class="text-lg">bpm</span>
            </div>
          {/if}
          {#if device.isConnected && device.type === DeviceType.SmartTrainer && device.bleDevice.data}
            {#if device.bleDevice.data.power !== null}
              <div class="text-white font-bold">
                <span class="text-5xl">{device.bleDevice.data.power}</span>
                <span class="text-lg">watt</span>
              </div>
            {/if}
            <div class="flex flex-row space-x-4">
              {#if device.bleDevice.data.speed !== null}
                <div class="text-white font-bold">
                  <span class="text-sm">{device.bleDevice.data.speed}</span>
                  <span class="text-sm">kph</span>
                </div>
              {/if}
              {#if device.bleDevice.data.cadence !== null}
                <div class="text-white font-bold">
                  <span class="text-sm">{device.bleDevice.data.cadence}</span>
                  <span class="text-sm">rpm</span>
                </div>
              {/if}
            </div>
          {/if}
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
    <ScanList
      scannedDevices="{scannedDevices}"
      handleConnect="{handleConnect}"
      handleCloseScan="{handleCloseScan}"
    />
  {/if}
</div>
