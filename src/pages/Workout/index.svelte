<script lang="ts">
// Libraries
import { createEventDispatcher, onMount } from 'svelte'
import { invoke } from '@tauri-apps/api/tauri'
import { listen, type Event as TauriEvent } from '@tauri-apps/api/event'

// Stores
import { devicesStore } from '../../stores/devices'
import { activityStore } from '../../stores/activities'
import { TimerStatus, useTimer } from '../../stores/useTimer'

// Components
import DataView from './components/DataView.svelte'
import Speed from './components/Speed.svelte'
import WorkoutsList from './components/WorkoutsList.svelte'
import PowerChart from './components/PowerChart.svelte'
import HeartRateChart from './components/HeartRateChart.svelte'
import Menu from './components/Menu.svelte'
import Summary from './components/Summary.svelte'
import SimulationActions from './components/SimulationActions.svelte'

// Types
import {
  DataType,
  DeviceType,
  type Activity,
  WorkoutType,
  DispatchMessage,
  Page,
  type SessionData,
} from '../../types'
import { convertSecondsToMinutes } from '../../utils/time'
import { getWorkoutData } from '../../utils/data'

// Styles
import './styles.css'

enum SessionStatus {
  Started = 'started',
  Stopped = 'stopped',
  Paused = 'paused',
}

enum StopAction {
  Stop = 'stop',
  Pause = 'pause',
}

const WORKOUT_START_INDEX = 0
const MAX_IDLE_TIME = 3
const IS_SIMULATED = import.meta.env.MODE === 'development'

const dispatch = createEventDispatcher()

const {
  elapsedTime,
  intervalTime,
  getStatus,
  start,
  stop,
  pause,
  resetInterval,
} = useTimer()

let activity: Activity
let activeWorkoutIndex = WORKOUT_START_INDEX
let currentPower: number
let displaySummary = false
let session = {
  status: SessionStatus.Stopped,
  idleTime: 0,
}
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

let sessionData: SessionData | null = null
let sessionDistance = 0

$: workoutData = getWorkoutData(activity, activeWorkoutIndex, $intervalTime)

onMount(() => {
  window.addEventListener('keydown', handleKeyPress)

  return () => {
    window.removeEventListener('keydown', handleKeyPress)
  }
})

listen('session_started', () => {
  // TODO: Handle session started
})

listen('session_stopped', (event: TauriEvent<any>) => {
  const { payload } = event

  // TODO: Handle session stopped/paused
})

const handleKeyPress = (event: KeyboardEvent) => {
  const { key } = event

  // TODO: Add keys to constants
  if (key === 'Escape') {
    pauseSession()
  }
}

const trackSessionState = async () => {
  const speed = devices[DataType.Speed].value
  const power = devices[DataType.Power].value

  const hasActivity = !!speed && !!power
  const isIdle = !speed && !power

  const { status, idleTime } = session

  if (
    (status === SessionStatus.Stopped || status === SessionStatus.Paused) &&
    hasActivity
  ) {
    startSession()

    return
  }

  if (idleTime === MAX_IDLE_TIME) {
    pauseSession()

    return
  }

  if (status === SessionStatus.Started && isIdle) {
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

  if (hrm && hrm.bleDevice && hrm.bleDevice.data) {
    const { bpm, is_sensor_in_contact } = hrm.bleDevice.data
    devices[DataType.HeartRate].value = is_sensor_in_contact ? bpm : '--'
  }

  if (smartTrainer && smartTrainer.bleDevice && smartTrainer.bleDevice.data) {
    const { cadence, distance, power, speed } = smartTrainer.bleDevice.data

    if (sessionDistance < distance) {
      sessionDistance = distance
    }

    devices[DataType.Distance].value = sessionDistance
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
    handleEndSession()

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

const startSession = async () => {
  session = {
    ...session,
    status: SessionStatus.Started,
  }

  const status = getStatus()

  if (status === TimerStatus.Started) {
    return
  }

  activeWorkoutIndex = WORKOUT_START_INDEX

  const action = IS_SIMULATED ? 'start_simulated_session' : 'start_session'

  await invoke(action)
  await executeWorkout()

  start()
}

const pauseSession = async () => {
  session = {
    status: SessionStatus.Paused,
    idleTime: 0,
  }

  pause()

  try {
    const action = IS_SIMULATED ? 'stop_simulated_session' : 'stop_session'

    await invoke(action, { action: StopAction.Pause })
  } catch (error) {
    // TODO: Render an exit dialog
    // Redirect to activities
    dispatch(DispatchMessage.PageChange, {
      page: Page.Activities,
    })
  }
}

const stopSession = async () => {
  session = {
    status: SessionStatus.Stopped,
    idleTime: 0,
  }

  stop()

  try {
    const action = IS_SIMULATED ? 'stop_simulated_session' : 'stop_session'

    await invoke(action, { action: StopAction.Stop })
  } catch (error) {
    // TODO: Handle if error
  }
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

  if (IS_SIMULATED) {
    await invoke('set_simulation_targets', {
      cadence,
      power,
    })

    return
  }

  await invoke('execute_workout', {
    cadence,
    power,
  })
}

const handleEndSession = async () => {
  await pauseSession()

  const action = IS_SIMULATED
    ? 'get_simulated_session_data'
    : 'get_session_data'

  sessionData = await invoke(action)

  displaySummary = true
}

// This is only used for development simulation
const handleStartSimulatedSession = async () => {
  await invoke('start_simulation')
}
const handleStopSimulatedSession = async (action = 'pause') => {
  await invoke('stop_simulation', { action })

  // TODO: If action is `stop`, stop the session, simulate ending of workout
}

const handleSaveSession = async () => {
  await stopSession()

  // TODO: Save summary details

  displaySummary = false

  // TODO: Add option for user to not save the session

  dispatch(DispatchMessage.PageChange, {
    page: Page.Activities,
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

  <div
    class="charts absolute bottom-12 left-12 right-60 flex flex-col space-y-4"
  >
    <div class="relative h-48 rounded-lg bg-gray-100 p-4">
      <div class="chart-name absolute left-4 top-4 font-bold">Heart Rate</div>
      <HeartRateChart
        activity="{activity}"
        elapsedTime="{elapsedTime}"
        devices="{devices}"
      />
    </div>
    <div class="relative h-48 rounded-lg bg-gray-100 p-4">
      <div class="chart-name absolute left-4 top-4 font-bold">Power</div>
      <PowerChart
        activity="{activity}"
        elapsedTime="{elapsedTime}"
        devices="{devices}"
      />
    </div>
  </div>

  <Speed devices="{devices}" />

  {#if IS_SIMULATED}
    <SimulationActions
      started="{session.status == SessionStatus.Started}"
      onStart="{handleStartSimulatedSession}"
      onStop="{handleStopSimulatedSession}"
    />
  {/if}

  {#if session.status === SessionStatus.Paused && !displaySummary}
    <Menu onSessionEnd="{handleEndSession}" />
  {/if}

  {#if displaySummary}
    <Summary
      sessionData="{sessionData}"
      onBackClick="{() => (displaySummary = false)}"
      onSaveClick="{handleSaveSession}"
    />
  {/if}
</div>
