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

enum SessionStatus {
  Started = 'started',
  Stopped = 'stopped',
  Paused = 'paused',
}

const WORKOUT_START_INDEX = 0
const MAX_IDLE_TIME = 3

let activity: Activity
let activeWorkoutIndex = WORKOUT_START_INDEX
let currentPower: number
let session = {
  status: SessionStatus.Stopped,
  idleTime: 0,
}

const {
  elapsedTime,
  intervalTime,
  getStatus,
  start,
  stop,
  pause,
  resetInterval,
} = useTimer()

$: workoutData = getWorkoutData(activity, activeWorkoutIndex, $intervalTime)

let devices = {
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
    value: 0,
    unit: 'watt',
  },
  [DataType.Cadence]: {
    value: 0,
    unit: 'rpm',
  },
  [DataType.TargetCadence]: {
    value: 0,
    unit: 'rpm',
  },
  [DataType.IntervalTime]: {
    value: '--',
    unit: '',
  },
  [DataType.ElapsedTime]: {
    value: '--',
    unit: '',
  },
}

const trackSessionState = () => {
  const speed = devices[DataType.Speed].value
  const power = devices[DataType.Power].value

  const hasActivity = !!speed && !!power
  const isIdle = !speed && !power

  const { status, idleTime } = session

  if (
    (status === SessionStatus.Stopped || status === SessionStatus.Paused) &&
    hasActivity
  ) {
    session = {
      ...session,
      status: SessionStatus.Started,
    }

    startWorkout()
    // TODO: Start FTMS workout

    return
  }

  if (status === SessionStatus.Started && isIdle) {
    if (idleTime === MAX_IDLE_TIME) {
      session = {
        status: SessionStatus.Paused,
        idleTime: 0,
      }

      pause()
      // TODO: Pause workout

      return
    }

    session = {
      ...session,
      idleTime: idleTime + 1,
    }

    return
  }
}

devicesStore.subscribe((map) => {
  const hrm = map[DeviceType.HeartRate]
  const smartTrainer = map[DeviceType.SmartTrainer]

  if (!devices) {
    return
  }

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

    trackSessionState()
  }
})

activityStore.subscribe((value) => (activity = value))

intervalTime.subscribe(async (time) => {
  if (!activity || !workoutData) {
    return
  }

  setTimes()

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

const setTimes = () => {
  if (!activity) {
    return
  }

  const activeWorkout = activity.workouts[activeWorkoutIndex]

  if (!activeWorkout) {
    return
  }

  const timeRemaining = activeWorkout.duration - $intervalTime

  devices[DataType.IntervalTime].value =
    convertSecondsToMinutes(timeRemaining).formatted
  devices[DataType.ElapsedTime].value =
    convertSecondsToMinutes($elapsedTime).formatted
}

const startWorkout = async () => {
  const status = getStatus()

  if (status === TimerStatus.Started) {
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

  devices[DataType.TargetPower].value = power
  devices[DataType.TargetCadence].value = cadence

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

  <div class="basis-1/2">
    <DataView devices="{devices}" />
  </div>

  <div class="basis-3/12 pl-4">
    <WorkoutsList
      activity="{activity}"
      elapsedTime="{elapsedTime}"
      intervalTime="{intervalTime}"
      activeWorkoutIndex="{activeWorkoutIndex}"
    />
  </div>

  <Speed devices="{devices}" />
</div>
