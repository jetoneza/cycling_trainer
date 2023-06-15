<script lang="ts">
// Types
import type { BasicObject } from 'src/types'

// Utils
import clickOutside from '../../../utils/clickOutside'

export let scannedDevices: Array<BasicObject>
export let handleConnect: (device: BasicObject) => {}
export let handleCloseScan: () => {}
</script>

<div class="scanned-devices-list overflow-hidden" use:clickOutside>
  <div class="animate-pulse title">Scanning</div>
  <div class="list-container">
    {#each scannedDevices as device}
      <button
        class="device-item {device.isConnecting ? 'text-primary-400' : ''}"
        on:click="{() => handleConnect(device)}"
      >
        {device.name}
        {#if device.isConnecting}
          <div class="text-sm text-secondary-100 animate-pulse">
            Connecting...
          </div>
        {/if}
      </button>
    {/each}
  </div>
  <button on:click="{() => handleCloseScan()}" class="btn close-btn"
    >Close</button
  >
</div>
