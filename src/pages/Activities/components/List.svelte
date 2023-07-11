<script lang="ts">
// Icons
import ClockIcon from 'svelte-icons/fa/FaRegClock.svelte'

// Types
import type { Activity } from 'src/types'

// Constants
const MINUTE = 60 // seconds

// Props
export let activities: Array<Activity>
export let selectedActivity: Activity
export let handleSelectActivity: (activity: Activity) => {}

const getDuration = (activity: Activity) => {
  const { workouts } = activity

  return workouts.reduce((acc, workout) => workout.duration + acc, 0) / MINUTE
}
</script>

<div class="list flex-1 p-6 space-y-2">
  {#each activities as activity}
    <button
      class="{selectedActivity.id === activity.id
        ? 'selected'
        : ''} activity-item relative flex flex-col py-4 px-6 w-full border rounded-lg"
      on:click="{() => handleSelectActivity(activity)}"
    >
      <span class="font-bold">{activity.name}</span>
      <span class="text-sm">{activity.description}</span>
      <span class="absolute right-6 top-4 flex font-bold text-sm">
        <div class="icon w-5 h-5 mr-2">
          <ClockIcon />
        </div>
        {getDuration(activity)} mins
      </span>
    </button>
  {/each}
</div>
