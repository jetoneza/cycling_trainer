<script lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
import { listen, type Event as TauriEvent } from '@tauri-apps/api/event'

let scanning = false
let stopping = false
let devices = []

async function scanDevices() {
  scanning = true

  await invoke('scan_devices')

  scanning = false
}

listen('devices-discovered', (event: TauriEvent<any>) => {
  const { payload } = event

  devices = payload
})
</script>

<div>
  <div class="row">
    {#if !scanning}
      <button on:click="{scanDevices}">Scan devices</button>
    {:else}
      <button on:click="{stopScanning}" disabled={stopping}>{stopping ? 'Stopping Scan...' : 'Stop Scan'}</button>
    {/if}
  </div>

  {#if scanning}
    <p>Searching via Bluetooth...</p>
  {/if}

  {#if devices.length > 0}
    <p>Devices found:</p>
    <ul>
      {#each devices as device}
        <li>
          {device.name}
        </li>
      {/each}
    </ul>
  {/if}
</div>
