<style>
.devices-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.devices-list .device {
  background: rgba(255, 0, 0, 0.25);
  border-radius: 1rem;
  padding: 1rem;
}

.devices-list .device.is-connected {
  background: rgba(0, 255, 0, 0.25);
}

.devices-list .actions {
  margin-top: 1rem;
}
</style>

<script lang="ts">
interface Device {
  id: string
  title: string
  name?: string
  isConnected: boolean
}

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

async function handleAction(selectedDevice: Device) {
  devices = devices.map((device) => {
    if (device.id == selectedDevice.id) {
      return {
        ...device,
        isConnected: !selectedDevice.isConnected,
      }
    }

    return device
  })
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
</div>
