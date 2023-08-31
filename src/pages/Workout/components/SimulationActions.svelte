<script lang="ts">
import PlayIcon from 'svelte-icons/fa/FaPlay.svelte'
import StopIcon from 'svelte-icons/fa/FaStop.svelte'
import PauseIcon from 'svelte-icons/fa/FaPause.svelte'

export let onStart: () => void
export let onStop: (action: string) => void
export let started = false

function handlePlayPause() {
  if (started) {
    onStop('pause')
    return
  }

  onStart()
}
</script>

<div
  class="absolute left-7 top-7 flex justify-center overflow-hidden rounded-lg border-2 border-black p-4 shadow-lg"
>
  <button class="border-r-2 border-gray-500 px-6" on:click="{handlePlayPause}">
    <div class="icon w-5">
      {#if started}
        <PauseIcon />
      {:else}
        <PlayIcon />
      {/if}
    </div>
  </button>
  <button class="px-6" on:click="{() => onStop('stop')}">
    <div class="icon w-5">
      <StopIcon />
    </div>
  </button>
</div>
