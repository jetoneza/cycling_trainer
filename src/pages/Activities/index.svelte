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

  console.log(activities)

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
</script>

<div class="activities mt-4 flex">
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
