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
  console.log('Staring Activity: ', selectedActivity.id)

  dispatch(DispatchMessage.PageChange, {
    page: Page.Workout,
  })
}
</script>

<div class="activities flex mt-4">
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
