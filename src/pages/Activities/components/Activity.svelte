<script lang="ts">
import type { Activity } from 'src/types'

export let selectedActivity: Activity
</script>

{#if !!selectedActivity}
  <div class="activity-viewer flex-1 p-6">
    <div class="title text-2xl font-bold">{selectedActivity.name}</div>
    <div class="decription">{selectedActivity.description}</div>
  </div>
{/if}
