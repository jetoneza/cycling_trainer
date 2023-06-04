<style>
.page-title {
  text-align: center;
}

.devices-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.devices-list .device {
  border: 2px solid var(--black);
  padding: 1rem;
}

.devices-list .title {
  margin: 0;
}

.devices-list .actions {
  margin-top: 1rem;
}

.scanned-devices-list {
  position: absolute;
  left: 0;
  right: 0;
  top: 0;
  bottom: 0;
  margin: 2rem auto;

  background: var(--white);
  color: var(--black);

  width: 50%;
  box-shadow: 0 0 0 50vmax rgba(0, 0, 0, 0.5);
}

.scanned-devices-list .title {
  padding: 0;
  margin: 0;
  background: var(--black);
  color: var(--white);
  padding-top: 0.5rem;
  padding-bottom: 0.5rem;
  text-align: center;
}

.scanned-devices-list .list-container .device {
  padding: 1rem;
  border-bottom: 2px solid var(--black);
  cursor: pointer;
  font-weight: bold;
}

.scanned-devices-list .list-container .device:hover {
  background: rgba(0, 0, 0, 0.1);
}

.scanned-devices-list .close-btn {
  position: absolute;
  bottom: 1rem;
  left: 50%;
  transform: translateX(-50%);
}
</style>

<script lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
import { listen, type Event as TauriEvent } from '@tauri-apps/api/event'

// Utils
import clickOutside from '../utils/clickOutside'

interface Device {
  id: string
  title: string
  name?: string
  isConnected: boolean
}

// States
let isScanning = false
let scannedDevices = []
let devices: Array<Device> = [
  {
    id: 'hrm',
    title: 'Heart Rate Monitor',
    isConnected: false,
  },
  {
    id: 'sc',
    title: 'Smart trainer (Power Meter, Speed, and Cadence)',
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
    disconnectDevice(device.id)

    return
  }

  // Start scanning
  isScanning = true

  await invoke('start_scan')
}

async function handleConnect(deviceID: string) {
  // TODO: Implement connection here

  changeConnectionState(deviceID, true)

  isScanning = false
}

async function disconnectDevice(deviceID: string) {
  // TODO: Handle disconnection here

  changeConnectionState(deviceID, false)
}

async function changeConnectionState(deviceID: string, isConnected: boolean) {
  devices = devices.map((device) => {
    if (device.id == deviceID) {
      return {
        ...device,
        isConnected,
      }
    }

    return device
  })
}

async function handleCloseScan() {
  await invoke('stop_scan')

  isScanning = false
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
          Name: {device.name || '--'}
        </div>
        <div class="actions">
          <button on:click="{() => handleAction(device)}"
            >{device.isConnected ? 'Disconnect' : 'Connect'}</button
          >
        </div>
      </div>
    {/each}
  </div>

  {#if isScanning}
    <div class="scanned-devices-list" use:clickOutside>
      <h3 class="title">Scanning...</h3>
      <div class="list-container">
        {#each scannedDevices as device}
          <div class="device" on:click="{() => handleConnect(device.id)}">
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
