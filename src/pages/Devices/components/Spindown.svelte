<script lang="ts">
// Libraries
import { invoke } from '@tauri-apps/api/tauri'
import { listen, type Event as TauriEvent } from '@tauri-apps/api/event'

// Types
import { type Device } from '../../../types'

// Utils
import clickOutside from '../../../utils/clickOutside'

enum Status {
  Pending,
  Started,
  StopPedal,
  Done,
}

const MESSAGES = {
  [Status.Pending]: {
    header: '',
    body: 'Perform a calibration spin-down regularly for accurate measurements. Allow the machine to warm up by riding it for at least 10 minutes before starting the spin-down process.',
    action: 'Start',
  },
  [Status.Started]: {
    header: (speed: number) =>
      speed ? `Accelerate to ${speed}kph` : 'Getting target speed...',
    body: 'Pedal until you reach the target speed. Maintain your speed until you receive a signal to stop pedaling.',
    action: null,
  },
  [Status.StopPedal]: {
    header: 'Stop Pedaling',
    body: 'Let the bike come to a complete stop.',
    action: null,
  },
  [Status.Done]: {
    header: 'Calibration Successful!',
    body: 'Congratulations! Your indoor cycling trainer is now ready. Enjoy your workouts!',
    action: 'Done',
  },
}

export let device: Device
export let handleToggleSpindown: (action: boolean) => {}

let calibrationStatus = Status.Pending
let targetSpeed = 0
let displaySpeed = false

$: speed = device.bleDevice.data.speed

listen('spin_down_start', (event: TauriEvent<any>) => {
  const { payload } = event

  targetSpeed = payload / 100
  displaySpeed = true
})

listen('spin_down_stop_pedaling', () => {
  calibrationStatus = Status.StopPedal
})

listen('spin_down_success', () => {
  calibrationStatus = Status.Done
})

const handleAction = async () => {
  if (calibrationStatus === Status.Pending) {
    calibrationStatus = Status.Started

    await invoke('request_spin_down')

    return
  }

  if (calibrationStatus == Status.Done) {
    handleToggleSpindown(false)

    return
  }
}
</script>

<div class="spindown absolute overflow-hidden" use:clickOutside>
  <div class="title">Calibrate Spindown</div>
  <div class="content mt-12 p-4 text-center">
    {#if displaySpeed && speed !== null}
      <div
        class="speed font-bold {speed >= targetSpeed ||
        calibrationStatus !== Status.Started
          ? 'text-primary-400'
          : ''}"
      >
        <div class="text-9xl">{speed}</div>
        <span class="text-2xl">kph</span>
      </div>
    {/if}

    <div class="header mt-4 text-lg font-bold">
      {calibrationStatus == Status.Started
        ? MESSAGES[calibrationStatus].header(targetSpeed)
        : MESSAGES[calibrationStatus].header}
    </div>

    <p class="status mt-4 px-6 text-justify indent-10">
      {MESSAGES[calibrationStatus].body}
    </p>
  </div>

  {#if !!MESSAGES[calibrationStatus].action}
    <button class="btn btn-close" on:click="{handleAction}"
      >{MESSAGES[calibrationStatus].action}</button
    >
  {/if}
</div>
