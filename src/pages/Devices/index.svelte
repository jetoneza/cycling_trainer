<script lang="ts">
// Libraries
import { invoke } from '@tauri-apps/api/tauri'
import { listen, type Event as TauriEvent } from '@tauri-apps/api/event'

// Components
import ScanList from './components/ScanList.svelte'
import DeviceCard from './components/DeviceCard.svelte'

// Stores
import { devicesStore, updateDevices } from '../../stores/devices'

// Types
import { DeviceType, type Device } from '../../types'

// Styles
import './styles.css'

// States
let isScanning = false
let isConnecting = false
let scannedDevices = []

listen('device_discovered', (event: TauriEvent<any>) => {
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
  if (isScanning) {
    return
  }

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

    updateDevices((map) => {
      const device = map[type]

      map[type] = {
        ...device,
        bleDevice: {
          id,
          name,
        },
        isConnected: true,
      }

      return map
    })
  })

  handleCloseScan()
}

async function disconnectDevice(device: Device) {
  await invoke('disconnect_device', { deviceId: device.bleDevice.id })

  changeConnectionState(device.type, false)
}

async function changeConnectionState(type: DeviceType, isConnected: boolean) {
  updateDevices((map) => {
    const device = map[type]

    map[type] = {
      ...device,
      isConnected,
      bleDevice: isConnected ? device.bleDevice : null,
    }

    return map
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

<div class="devices-page p-10">
  <div class="page-title text-center text-2xl font-bold">Devices</div>

  <div class="devices-list m-10 flex justify-center space-x-6">
    <DeviceCard
      device="{$devicesStore[DeviceType.HeartRate]}"
      handleAction="{handleAction}"
    />
    <DeviceCard
      device="{$devicesStore[DeviceType.SmartTrainer]}"
      handleAction="{handleAction}"
    />
  </div>

  {#if isScanning}
    <ScanList
      scannedDevices="{scannedDevices}"
      handleConnect="{handleConnect}"
      handleCloseScan="{handleCloseScan}"
    />
  {/if}
</div>
