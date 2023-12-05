<script lang="ts">
import { onMount, createEventDispatcher } from 'svelte'
import { invoke } from '@tauri-apps/api/tauri'

// Components
import List from './components/List.svelte'
import ActivityComponent from './components/Activity.svelte'

// Types
import { DispatchMessage, type Activity, Page } from '../../types'

// Styles
import './styles.css'
import { activityStore } from '../../stores/activities'

const dispatch = createEventDispatcher()

let activities: Array<Activity> = []
let selectedActivity: Activity

onMount(async () => {
  activities = await invoke('get_activities')

  selectedActivity = activities[0]
})

const handleSelectActivity = (activity: Activity) =>
  (selectedActivity = activity)

const handleStartActivity = () => {
  activityStore.set({
    ...selectedActivity,

    // TODO: Use dynamic fpt value
    // TODO: Remove static ftp value
    ftp: 200,
  })

  dispatch(DispatchMessage.PageChange, {
    page: Page.Workout,
  })
}

const handleCreateWorkout = () => {
  // TODO: Toggle create workout
}
</script>

<div class="activities mt-4 flex">
  {#if activities.length == 0}
    <div class="activity-viewer flex flex-1 flex-col space-y-4 p-6">
      <div class="title-wrapper">
        <div class="title text-2xl font-bold">No Workouts</div>
        <div class="decription">Create your very first workout!</div>
      </div>

      <div class="actions">
        <button class="btn text-md" on:click="{handleCreateWorkout}">
          Create Workout
        </button>
      </div>
    </div>
  {/if}

  <ActivityComponent
    selectedActivity="{selectedActivity}"
    handleStartActivity="{handleStartActivity}"
  />
  <List
    activities="{activities}"
    selectedActivity="{selectedActivity}"
    handleSelectActivity="{handleSelectActivity}"
  />
</div>
