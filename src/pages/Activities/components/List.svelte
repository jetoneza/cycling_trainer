<script lang="ts">
// Icons
import ClockIcon from 'svelte-icons/fa/FaRegClock.svelte'

// Types
import type { Activity } from 'src/types'
import {
  convertSecondsToMinutes,
  getActivityDuration,
} from '../../../utils/time'

// Props
export let activities: Array<Activity>
export let selectedActivity: Activity
export let handleSelectActivity: (activity: Activity) => {}
</script>

<div class="activities-list scrollable flex-1 space-y-2 p-6">
  {#each activities as activity}
    <button
      class="{selectedActivity.id === activity.id
        ? 'selected'
        : ''} activity-item relative flex w-full flex-col rounded-lg border px-6 py-4"
      on:click="{() => handleSelectActivity(activity)}"
    >
      <span class="font-bold">{activity.name}</span>
      <span class="text-sm">{activity.description}</span>
      <span class="absolute right-6 top-4 flex text-sm font-bold">
        <div class="icon mr-2 h-5 w-5">
          <ClockIcon />
        </div>
        {convertSecondsToMinutes(getActivityDuration(activity)).formatted} mins
      </span>
    </button>
  {/each}
</div>
