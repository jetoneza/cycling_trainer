<script lang="ts">
import PlayIcon from 'svelte-icons/fa/FaPlay.svelte'
import StopIcon from 'svelte-icons/fa/FaStop.svelte'

// export const onSessionStop = () => {}
export let onSessionStop: () => void
</script>

<div class="menu flex justify-center overflow-hidden">
  <button class="border-r-2 border-gray-500 px-6">
    <div class="icon w-10">
      <PlayIcon />
    </div>
  </button>
  <button class="px-6" on:click="{onSessionStop}">
    <div class="icon w-10">
      <StopIcon />
    </div>
  </button>
</div>
