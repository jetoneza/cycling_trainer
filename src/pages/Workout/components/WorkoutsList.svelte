<script lang="ts">
import type { Writable } from 'svelte/store'

// Types
import { WorkoutType, type Activity, type Workout } from '../../../types'

// Utils
import {
  convertSecondsToMinutes,
  getActivityDuration,
} from '../../../utils/time'

export let activity: Activity
export let elapsedTime: Writable<number>
export let intervalTime: Writable<number>
export let activeWorkoutIndex: number

$: activeWorkout = activity && activity.workouts[activeWorkoutIndex]

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

$: getTimeRemaining = () => {
  const activityDuration = getActivityDuration(activity)

  const timeRemaining = activityDuration - $elapsedTime

  const { formatted } = convertSecondsToMinutes(timeRemaining)

  return `${formatted}`
}

$: getWorkoutCompletion = () =>
  Math.floor(($intervalTime / activeWorkout.duration) * 100)
</script>

<div class="workouts-list">
  <div class="flex flex-col px-4 py-2 text-right text-secondary-200">
    <span class="font-bold">Workout {activeWorkoutIndex + 1}/{activity.workouts.length}</span>
    <span class="text-xs">{getTimeRemaining()} remaining</span>
  </div>
  {#if !!activeWorkout}
    <div
      class="progress-wrapper relative border border-primary-100 p-1 px-2 text-right"
    >
      <span
        class="progress absolute bottom-0 left-0 top-0 bg-primary-100"
        style="width: {getWorkoutCompletion()}%"></span>
      <span class="z-10 text-sm font-bold leading-3 text-primary-400">
        {format(activeWorkout)}
      </span>
    </div>
  {/if}
</div>
