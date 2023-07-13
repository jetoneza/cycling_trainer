<script lang="ts">
import type { Activity } from 'src/types'

export let selectedActivity: Activity
export let handleStartActivity: () => void
</script>

{#if !!selectedActivity}
  <div class="activity-viewer flex flex-1 flex-col space-y-4 p-6">
    <div class="title-wrapper">
      <div class="title text-2xl font-bold">{selectedActivity.name}</div>
      <div class="decription">{selectedActivity.description}</div>
    </div>

    <div class="actions">
      <button class="btn text-md" on:click="{handleStartActivity}"
        >Start Workout</button
      >
    </div>
  </div>
{/if}
