import { WorkoutType, type Activity, type Workout } from '../types'

interface Data {
  power: number
  cadence: number
}

const POWER_JUMP = 5

export const getWorkoutData = (
  activity: Activity,
  activeWorkoutIndex: number,
  intervalTime: number
): Data => {
  const data: Data = {
    power: 0,
    cadence: 0,
  }

  if (!activity) {
    return data
  }

  const { ftp, workouts } = activity

  const workout = workouts[activeWorkoutIndex]

  if (!workout) {
    return data
  }

  data.power = calculatePower(workout, ftp, intervalTime)
  data.cadence = workout.cadence || 0

  return data
}

const calculatePower = (
  workout: Workout,
  ftp: number,
  intervalTime: number
): number => {
  const { workoutType, powerSteady, powerLow, powerHigh, duration } = workout
  if (workoutType === WorkoutType.SteadyState) {
    return Math.floor(powerSteady * ftp)
  }

  return calculateRangePower(ftp, powerLow, powerHigh, duration, intervalTime)
}

const calculateRangePower = (
  ftp: number,
  low: number,
  high: number,
  duration: number,
  elapsedTime: number
): number => {
  const lowPower = Math.floor(low * ftp)
  const highPower = Math.floor(high * ftp)

  const range = Math.abs(highPower - lowPower)
  const timeJump = Math.floor(duration / Math.floor(range / POWER_JUMP))
  const accumulator = POWER_JUMP * Math.floor(elapsedTime / timeJump)

  let power = lowPower + accumulator

  if (low > high) {
    power = lowPower - accumulator
  }

  return Math.round(power / POWER_JUMP) * POWER_JUMP
}
