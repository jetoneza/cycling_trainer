<script lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
import { listen, type Event as TauriEvent } from '@tauri-apps/api/event'

let scanning = false
let connecting = false
let devices = {
  scanned: [],
  connected: [],
}

async function startScan() {
  scanning = true

  await invoke('start_scan')
}

async function stopScan() {
  await invoke('stop_scan')

  scanning = false
  devices.scanned = []
}

async function connectToDevice(deviceId: string) {
  connecting = true

  await invoke('connect_to_device', { deviceId })

  connecting = false

  stopScan()
}

listen('device-discovered', (event: TauriEvent<any>) => {
  const { payload } = event

  const [id, name] = payload

  const exising = devices.scanned.find((d) => d.id == id)

  if (exising) {
    return
  }

  devices.scanned = [
    ...devices.scanned,
    {
      id,
      name,
    },
  ]
})

async function getConnectedDevices() {
  const connectedDevices: Array<[string, string]> = await invoke(
    'get_connected_devices'
  )

  devices.connected = connectedDevices.map(([id, name]) => ({
    id,
    name,
  }))
}
</script>

<div>
  <div class="row">
    {#if !scanning}
      <button on:click="{startScan}">Scan devices</button>
    {/if}

    <button on:click="{getConnectedDevices}">Get devices</button>
  </div>

  {#if scanning}
    <p>Searching via Bluetooth...</p>
    <button on:click="{stopScan}">Stop Scan</button>
  {/if}

  {#if connecting}
    <p>Connecting...</p>
  {/if}

  {#if devices.scanned.length > 0}
    <p>Devices found:</p>
    <ul>
      {#each devices.scanned as device}
        <li>
          {device.name} [{device.id}]
          <button on:click="{() => connectToDevice(device.id)}">Connect</button>
        </li>
      {/each}
    </ul>
  {/if}

  {#if devices.connected.length > 0}
    <p>Connected devices:</p>
    <ul>
      {#each devices.connected as device}
        <li>
          {device.name} [{device.id}]
        </li>
      {/each}
    </ul>
  {:else}
    <p>No devices connected</p>
  {/if}
</div>
