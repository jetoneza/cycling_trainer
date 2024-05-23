import { WorkoutType, type Activity, type Workout } from '../types'

type Data = {
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

  const power = calculatePower(workout, ftp, intervalTime);

  data.power = (power === -Infinity) ? 0 : power || 0;
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
    const power = Math.floor(powerSteady * ftp)

    return setToNearestPowerJump(power)
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

  return setToNearestPowerJump(power)
}

const setToNearestPowerJump = (power: number): number =>
  Math.round(power / POWER_JUMP) * POWER_JUMP

export const formatIndoorBikeData = (
  data: Array<{ cadence: number; power: number; speed: number }>
) => {
  const formattedData: {
    cadenceData: number[]
    powerData: number[]
    speedData: number[]
  } = {
    cadenceData: [],
    powerData: [],
    speedData: [],
  }

  data.forEach(({ cadence, power, speed }) => {
    formattedData.cadenceData.push(cadence)
    formattedData.powerData.push(power)
    formattedData.speedData.push(speed)
  })

  return formattedData
}
