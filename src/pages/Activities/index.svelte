<script lang="ts">
import { onMount } from 'svelte'
import { invoke } from '@tauri-apps/api/tauri'

let workouts = []

onMount(async () => {
  workouts = await invoke('get_workouts')
})
</script>

<div class="activities">
  <h1>Workouts</h1>
  <button>Create workout</button>
</div>
