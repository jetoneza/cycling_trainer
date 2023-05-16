<script lang="ts">
import { invoke } from '@tauri-apps/api/tauri'

let query = ''
let result = ''
let searching = false

const resetState = () => {
  searching = true
  result = ''
}

async function findDevice() {
  resetState()

  result = result = await invoke('find_device', { query })

  searching = false
}
</script>

<div>
  <div class="row">
    <input
      id="greet-input"
      placeholder="Enter device..."
      bind:value="{query}"
    />
    <button on:click="{findDevice}"> Find </button>
  </div>
  {#if searching}
    <p>Searching via Bluetooth...</p>
  {/if}

  <p>{result}</p>
</div>
