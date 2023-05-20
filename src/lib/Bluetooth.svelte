<script lang="ts">
import { invoke } from '@tauri-apps/api/tauri'

let result = ''
let searching = false

const resetState = () => {
  searching = true
  result = ''
}

async function listDevices() {
  resetState()

  result = result = await invoke('list_devices')

  searching = false
}
</script>

<div>
  <div class="row">
    <button on:click="{listDevices}">Bluetooth</button>
  </div>

  {#if searching}
    <p>Searching via Bluetooth...</p>
  {/if}

  <p>{result}</p>
</div>
