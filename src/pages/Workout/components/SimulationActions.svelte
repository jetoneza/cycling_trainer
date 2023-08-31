<script lang="ts">
import PlayIcon from 'svelte-icons/fa/FaPlay.svelte'
import StopIcon from 'svelte-icons/fa/FaStop.svelte'

export let onStart: () => void
export let onStop: () => void
</script>

<div
  class="absolute left-7 top-7 flex justify-center overflow-hidden rounded-lg border-2 border-black p-4 shadow-lg"
>
  <button class="border-r-2 border-gray-500 px-6" on:click="{onStart}">
    <div class="icon w-5">
      <PlayIcon />
    </div>
  </button>
  <button class="px-6" on:click="{onStop}">
    <div class="icon w-5">
      <StopIcon />
    </div>
  </button>
</div>
