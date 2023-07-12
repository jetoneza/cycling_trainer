<script lang="ts">
// Types
import { WorkoutType, type Activity, type Workout } from '../../../types'

// Utils
import {
  convertSecondsToMinutes,
  getActivityDuration,
} from '../../../utils/time'

export let activity: Activity

const format = (workout: Workout) => {
  const { workoutType, powerSteady, cadence } = workout
  const { formatted } = convertSecondsToMinutes(workout.duration)

  if (
    workoutType === WorkoutType.Warmup ||
    workoutType === WorkoutType.Cooldown
  ) {
    return `${workoutType} for ${formatted}`
  }

  const power = Math.floor(powerSteady * activity.ftp)

  return `${power}w ${!!cadence ? `@ ${cadence}rpm` : ''} for ${formatted}`
}

const getTimeRemaining = () => {
  const activityDuration = getActivityDuration(activity)

  // TODO: Calculate time remaining

  const { formatted } = convertSecondsToMinutes(activityDuration)

  return `${formatted}`
}
</script>

<div class="workouts-list">
  {#if activity.workouts && activity.workouts.length > 0}
    <ul class="list-wrapper border border-primary-100 rounded-lg divide-y">
      <li
        class="workout font-bold text-right py-2 px-4 text-primary-400 border border-primary-100 bg-primary-100 rounded-t-lg"
      >
        Time remaining: {getTimeRemaining()}
      </li>
      {#each activity.workouts as workout}
        <li class="workout text-right py-1 px-4 {workout.status || ''}">
          {format(workout)}
        </li>
      {/each}
    </ul>
  {/if}
</div>
