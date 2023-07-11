<script lang="ts">
import { onMount } from 'svelte'
import { invoke } from '@tauri-apps/api/tauri'

// Components
import List from './components/List.svelte'
import ActivityComponent from './components/Activity.svelte'

// Types
import type { Activity } from '../../types'

// Styles
import './styles.css'

let activities: Array<Activity> = []
let selectedActivity: Activity

onMount(async () => {
  activities = await invoke('get_activities')

  selectedActivity = activities[0]
})

const handleSelectActivity = (activity: Activity) =>
  (selectedActivity = activity)
</script>

<div class="activities flex">
  <ActivityComponent selectedActivity="{selectedActivity}" />
  <List
    activities="{activities}"
    selectedActivity="{selectedActivity}"
    handleSelectActivity="{handleSelectActivity}"
  />
</div>
