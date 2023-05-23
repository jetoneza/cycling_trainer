<script lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
import { listen, type Event as TauriEvent } from '@tauri-apps/api/event'

let result = ''
let searching = false
let devices = []

const resetState = () => {
  searching = true
  result = ''
}

async function listDevices() {
  resetState()

  await invoke('list_devices')

  searching = false
}

listen('devices-discovered', (appEvent: TauriEvent<any>) => {
  const { payload } = appEvent

  devices = payload
})
</script>

<div>
  <div class="row">
    <button on:click="{listDevices}">Scan devices</button>
  </div>

  {#if searching}
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
