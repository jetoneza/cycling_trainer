<script lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
import { listen, type Event as TauriEvent } from '@tauri-apps/api/event'

let scanning = false
let connecting = false
let devices = []

async function startScan() {
  scanning = true

  await invoke('start_scan')
}

async function stopScan() {
  await invoke('stop_scan')

  scanning = false
  devices = []
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

  const exising = devices.find((d) => d.id == id)

  if (exising) {
    return
  }

  devices = [
    ...devices,
    {
      id,
      name,
    },
  ]
})
</script>

<div>
  <div class="row">
    {#if !scanning}
      <button on:click="{startScan}">Scan devices</button>
    {/if}
  </div>

  {#if scanning}
    <p>Searching via Bluetooth...</p>
    <button on:click="{stopScan}">Stop Scan</button>
  {/if}

  {#if connecting}
    <p>Connecting...</p>
  {/if}

  {#if devices.length > 0}
    <p>Devices found:</p>
    <ul>
      {#each devices as device}
        <li>
          {device.name} [{device.id}]
          <button on:click="{() => connectToDevice(device.id)}">Connect</button>
        </li>
      {/each}
    </ul>
  {/if}
</div>
