<style>
.devices-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.devices-list .device {
  background: rgba(255, 0, 0, 0.25);
  padding: 1rem;
}

.devices-list .device.is-connected {
  background: rgba(0, 255, 0, 0.25);
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

  background: white;
  color: black;

  width: 50%;
}

.scanned-devices-list .title {
  padding: 0;
  margin: 0;
  background: rgba(0, 0, 0, 0.9);
  color: white;
  padding-top: 0.5rem;
  padding-bottom: 0.5rem;
  text-align: center;
}

.scanned-devices-list .list-container .device {
  padding: 1rem;
  border-bottom: 2px solid black;
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
interface Device {
  id: string
  title: string
  name?: string
  isConnected: boolean
}

let isScanning = false

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

async function handleAction(device: Device) {
  if (device.isConnected) {
    disconnectDevice(device.id)

    return
  }

  isScanning = true
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
  isScanning = false
}
</script>

<div class="component-paired-devices">
  <h1>Paired Devices</h1>

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
    <div class="scanned-devices-list">
      <h3 class="title">Scanning...</h3>
      <div class="list-container">
        <div class="device" on:click="{() => handleConnect('sc')}">
          Wahoo Kickr
        </div>
        <div class="device" on:click="{() => handleConnect('hrm')}">Venu 2</div>
      </div>
      <button on:click="{() => handleCloseScan()}" class="close-btn"
        >Close</button
      >
    </div>
  {/if}
</div>
