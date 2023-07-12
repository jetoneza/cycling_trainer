<script lang="ts">
import HeartIcon from 'svelte-icons/fa/FaHeartbeat.svelte'
import LightningIcon from 'svelte-icons/fa/FaBolt.svelte'
import CadenceIcon from 'svelte-icons/go/GoSync.svelte'
import SpeedIcon from 'svelte-icons/io/IoMdSpeedometer.svelte'
import { DataType, DeviceType, type Activity } from '../../types'

// Stores
import { devicesStore } from '../../stores/devices'
import { activityStore } from '../../stores/activities'

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
    value: '05:34',
    unit: '',
  },
  [DataType.ElapsedTime]: {
    value: '45:31',
    unit: '',
  },
}

let activity: Activity

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
</script>

<div class="workout-page flex p-4 justify-between">
  <div class="basis-1/6">
    <!-- TODO: Add other data here. -->
  </div>

  <div class="main-data flex space-x-4 bg-secondary-200 rounded-lg">
    <div class="column flex flex-col justify-between text-center py-4 px-6">
      <div class="text-2xl font-bold text-white">Target</div>

      <div class="item">
        <div class="value font-bold text-4xl text-primary-300">
          {devices[DataType.TargetPower].value}<span class="text-2xl">w</span>
        </div>
      </div>

      {#if !!devices[DataType.TargetCadence]}
        <div class="text-2xl font-bold text-white">at</div>

        <div class="item">
          <div class="value font-bold text-4xl text-primary-300">
            {devices[DataType.TargetCadence].value}<span class="text-2xl"
              >rpm</span
            >
          </div>
        </div>
      {/if}
    </div>

    <div class="column pt-2 pb-4 px-6">
      <div class="row item text-center">
        <div
          class="value font-bold text-9xl flex w-full justify-center text-white"
        >
          {devices[DataType.Power].value}
          <div class="icon w-10 mt-11 ml-2">
            <LightningIcon />
          </div>
        </div>
      </div>

      <div class="row flex m-0 space-x-6 justify-center">
        <div class="item">
          <div class="value font-bold text-4xl flex text-white">
            {devices[DataType.HeartRate].value}
            <div class="icon h-5 w-5 mt-3 ml-2">
              <HeartIcon />
            </div>
          </div>
        </div>

        <div class="item">
          <div class="value font-bold text-4xl flex text-white">
            {devices[DataType.Cadence].value}
            <div class="icon h-5 w-5 mt-3 ml-1">
              <CadenceIcon />
            </div>
          </div>
        </div>
      </div>
    </div>

    <div class="column py-4 px-6 flex flex-col justify-between text-center">
      <div class="item text-white">
        <div class="text-xl font-bold">Elapsed</div>
        <div class="value font-bold text-4xl">
          {devices[DataType.ElapsedTime].value}
        </div>
      </div>
      <div class="item text-white">
        <div class="text-xl font-bold">Interval</div>
        <div class="value font-bold text-4xl">
          {devices[DataType.IntervalTime].value}
        </div>
      </div>
    </div>
  </div>

  <div class="basis-1/6">
    <!-- TODO: Add other data here. -->
  </div>

  <div class="absolute speed bottom-12 right-12 text-right">
    <div class="bg-secondary-200 rounded-lg px-4 py-2">
      <div class="font-bold text-white flex space-x-1 justify-end">
        <div class="text-6xl">
          {devices[DataType.Speed].value}
        </div>
        <div class="flex flex-col">
          <div class="icon w-10 h-10">
            <SpeedIcon />
          </div>
          <span class="text-md m-0 w-full text-center">kph</span>
        </div>
      </div>
      <div class="text-lg font-bold text-primary-300">
        {devices[DataType.Distance].value}<span class="text-lg ml-1"
          >km dist</span
        >
      </div>
    </div>
  </div>
</div>
