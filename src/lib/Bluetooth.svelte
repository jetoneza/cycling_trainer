<script lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
import { listen, type Event as TauriEvent } from '@tauri-apps/api/event'

let scanning = false
let connecting = false
let devices = []

async function startScan() {
  console.log('scanning')
  scanning = true

  await invoke('start_scan')

  scanning = false
}

async function connectToDevice(deviceId: string) {
  connecting = true

  await invoke('connect_to_device', { deviceId })

  connecting = false
}

listen('devices-discovered', (event: TauriEvent<any>) => {
  const { payload } = event

  devices = payload
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
  {/if}

  {#if connecting}
    <p>Connecting...</p>
  {/if}

  {#if devices.length > 0}
    <p>Devices found:</p>
    <ul>
      {#each devices as [deviceID, name]}
        <li>
          {name} [{deviceID}]
          <button on:click="{() => connectToDevice(deviceID)}">Connect</button>
        </li>
      {/each}
    </ul>
  {/if}
</div>
