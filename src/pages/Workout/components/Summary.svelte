<script lang="ts">
export let onBackClick: () => void
export let onSaveClick: () => void
</script>

<div class="summary">
  <div class="text-2xl text-center font-bold">Session Summary</div>
  <!-- TODO: Add summary graphs -->

  <div class="actions absolute bottom-12 flex w-full justify-center space-x-6">
    <button class="btn secondary w-1/6 text-2xl" on:click="{onBackClick}">
      Back
    </button>
    <button class="btn w-1/6 text-2xl" on:click="{onSaveClick}"> Save </button>
  </div>
</div>
