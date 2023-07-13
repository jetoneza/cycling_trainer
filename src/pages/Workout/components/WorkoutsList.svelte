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
    <ul class="list-wrapper divide-y rounded-lg border border-primary-100">
      <li
        class="workout rounded-t-lg border border-primary-100 bg-primary-100 px-4 py-2 text-right font-bold text-primary-400"
      >
        Time remaining: {getTimeRemaining()}
      </li>
      {#each activity.workouts as workout}
        <li class="workout px-4 py-1 text-right {workout.status || ''}">
          {format(workout)}
        </li>
      {/each}
    </ul>
  {/if}
</div>
