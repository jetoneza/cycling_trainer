<script lang="ts">
// Libraries
import { invoke } from '@tauri-apps/api/tauri'

// Stores
import { devicesStore } from '../../stores/devices'
import { activityStore } from '../../stores/activities'
import { TimerStatus, useTimer } from '../../stores/useTimer'

// Components
import DataView from './components/DataView.svelte'
import Speed from './components/Speed.svelte'
import WorkoutsList from './components/WorkoutsList.svelte'

// Types
import { DataType, DeviceType, type Activity, WorkoutType } from '../../types'
import { convertSecondsToMinutes } from '../../utils/time'
import { getWorkoutData } from '../../utils/data'

// Styles
import './styles.css'

const WORKOUT_START_INDEX = 0

let activity: Activity
let activeWorkoutIndex = WORKOUT_START_INDEX
let currentPower: number

const { elapsedTime, intervalTime, getStatus, start, stop, resetInterval } =
  useTimer()

$: workoutData = getWorkoutData(activity, activeWorkoutIndex, $intervalTime)

$: devices = {
  [DataType.Distance]: {
    value: 0,
    unit: 'km',
  },
  [DataType.Speed]: {
    value: 0,
    unit: 'kph',
  },
  [DataType.HeartRate]: {
    value: '--',
    unit: 'bpm',
  },
  [DataType.Power]: {
    value: 0,
    unit: 'watt',
  },
  [DataType.TargetPower]: {
    value: workoutData.power,
    unit: 'watt',
  },
  [DataType.Cadence]: {
    value: 0,
    unit: 'rpm',
  },
  [DataType.TargetCadence]: {
    value: workoutData.cadence,
    unit: 'rpm',
  },
  [DataType.IntervalTime]: {
    value: getIntervalTime(),
    unit: '',
  },
  [DataType.ElapsedTime]: {
    value: convertSecondsToMinutes($elapsedTime).formatted,
    unit: '',
  },
}

devicesStore.subscribe((map) => {
  const hrm = map[DeviceType.HeartRate]
  const smartTrainer = map[DeviceType.SmartTrainer]

  if (hrm && hrm.bleDevice) {
    const { bpm, is_sensor_in_contact } = hrm.bleDevice.data
    devices[DataType.HeartRate].value = is_sensor_in_contact ? bpm : '--'
  }

  if (smartTrainer && smartTrainer.bleDevice) {
    const { cadence, distance, power, speed } = smartTrainer.bleDevice.data

    devices[DataType.Distance].value = distance || 0
    devices[DataType.Speed].value = speed || 0
    devices[DataType.Power].value = power || 0
    devices[DataType.Cadence].value = cadence || 0
  }
})

activityStore.subscribe((value) => (activity = value))

intervalTime.subscribe(async (time) => {
  if (!activity) {
    return
  }

  const activeWorkout = activity.workouts[activeWorkoutIndex]

  if (time != activeWorkout.duration) {
    // TODO: Add restriction for warmup/cooldown if the duration is too short
    // This handles the change of power in a warmup/cooldown workout
    if (
      activeWorkout.workoutType !== WorkoutType.SteadyState &&
      currentPower !== workoutData.power
    ) {
      await executeWorkout()

      return
    }

    return
  }

  if (activeWorkoutIndex == activity.workouts.length - 1) {
    stop()

    return
  }

  activeWorkoutIndex++

  resetInterval()

  await executeWorkout()
})

const getIntervalTime = (): string => {
  if (!activity) {
    return '--'
  }

  const activeWorkout = activity.workouts[activeWorkoutIndex]

  if (!activeWorkout) {
    return '--'
  }

  const timeRemaining = activeWorkout.duration - $intervalTime

  return convertSecondsToMinutes(timeRemaining).formatted
}

const handleStartWorkout = async () => {
  const status = getStatus()

  if (status !== TimerStatus.Stopped) {
    return
  }

  activeWorkoutIndex = WORKOUT_START_INDEX

  await executeWorkout()

  start()
}

const executeWorkout = async () => {
  const { power, cadence } = getWorkoutData(
    activity,
    activeWorkoutIndex,
    $intervalTime
  )

  // Track change of power for warmup/cooldown
  currentPower = power

  await invoke('execute_workout', {
    cadence,
    power,
  })
}
</script>

<div class="workout-page flex justify-between py-4">
  <div class="basis-3/12">
    <!-- TODO: Add other data here. -->
  </div>

  <div class="main-data">
    <DataView devices="{devices}" />
  </div>

  <div class="basis-3/12">
    <WorkoutsList
      activity="{activity}"
      elapsedTime="{elapsedTime}"
      intervalTime="{intervalTime}"
      activeWorkoutIndex="{activeWorkoutIndex}"
    />
  </div>

  <Speed devices="{devices}" />

  <button
    on:click="{handleStartWorkout}"
    class="absolute bottom-8 left-8 rounded-lg border border-primary-400 bg-primary-400 p-4 font-bold text-white"
    >Start {$elapsedTime}</button
  >
</div>
