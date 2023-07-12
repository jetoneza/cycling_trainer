import type { Activity } from '../types'

const MINUTE = 60

export const convertSecondsToMinutes = (
  seconds: number
): {
  minutes: number
  seconds: number
  formatted: string
} => {
  const minutes = Math.floor(seconds / MINUTE)
  const remainingSeconds = seconds % MINUTE
  const formatted = `${String(minutes).padStart(2, '0')}:${String(
    remainingSeconds
  ).padStart(2, '0')}`

  return {
    minutes,
    seconds: remainingSeconds,
    formatted,
  }
}

export const getActivityDuration = (activity: Activity) => {
  const { workouts } = activity

  return workouts.reduce((acc, workout) => workout.duration + acc, 0)
}

export default {
  convertSecondsToMinutes,
  getActivityDuration,
}
