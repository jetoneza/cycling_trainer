<script lang="ts">
import { onMount } from 'svelte'
import { invoke } from '@tauri-apps/api/tauri'

let activities = []

onMount(async () => {
  activities = await invoke('get_activities')
})
</script>

<div class="activities-wrapper">
  <h1>Workouts</h1>

  <div class="activities mt-6">
    {#each activities as activity}
      <div class="activity p-6 border rounded">
        <span class="font-bold">{activity.name}</span> <br />
        <span class="text-sm">{activity.description}</span>
      </div>
    {/each}
  </div>
</div>
