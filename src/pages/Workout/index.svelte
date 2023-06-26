<script lang="ts">
import HeartIcon from 'svelte-icons/fa/FaHeartbeat.svelte'
import LightningIcon from 'svelte-icons/fa/FaBolt.svelte'
import CadenceIcon from 'svelte-icons/go/GoSync.svelte'
import SpeedIcon from 'svelte-icons/io/IoMdSpeedometer.svelte'
import { DataType, DeviceType } from '../../types'

// Stores
import { devices } from '../../stores/devices'

let data = {
  [DataType.Distance]: {
    value: 19.1,
    unit: 'km',
  },
  [DataType.Speed]: {
    value: 32,
    unit: 'kph',
  },
  [DataType.HeartRate]: {
    value: '--',
    unit: 'bpm',
  },
  [DataType.Power]: {
    value: 250,
    unit: 'watt',
  },
  [DataType.TargetPower]: {
    value: 245,
    unit: 'watt',
  },
  [DataType.Cadence]: {
    value: 90,
    unit: 'rpm',
  },
  [DataType.TargetCadence]: {
    value: 100,
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

devices.subscribe((items) => {
  // TODO: Use key value map for devices instead of array
  const hrm = items.find((item) => item.type === DeviceType.HeartRate)

  if (hrm) {
    const { bpm, is_sensor_in_contact } = hrm.bleDevice.data
    data[DataType.HeartRate].value = is_sensor_in_contact ? bpm : '--'
  }
})
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
          {data[DataType.TargetPower].value}<span class="text-2xl">w</span>
        </div>
      </div>

      {#if !!data[DataType.TargetCadence]}
        <div class="text-2xl font-bold text-white">at</div>

        <div class="item">
          <div class="value font-bold text-4xl text-primary-300">
            {data[DataType.TargetCadence].value}<span class="text-2xl">rpm</span
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
          {data[DataType.Power].value}
          <div class="icon w-10 mt-11 ml-2">
            <LightningIcon />
          </div>
        </div>
      </div>

      <div class="row flex m-0 space-x-6 justify-center">
        <div class="item">
          <div class="value font-bold text-4xl flex text-white">
            {data[DataType.HeartRate].value}
            <div class="icon h-5 w-5 mt-3 ml-2">
              <HeartIcon />
            </div>
          </div>
        </div>

        <div class="item">
          <div class="value font-bold text-4xl flex text-white">
            {data[DataType.Cadence].value}
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
          {data[DataType.ElapsedTime].value}
        </div>
      </div>
      <div class="item text-white">
        <div class="text-xl font-bold">Interval</div>
        <div class="value font-bold text-4xl">
          {data[DataType.IntervalTime].value}
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
          {data[DataType.Speed].value}
        </div>
        <div class="flex flex-col">
          <div class="icon w-10 h-10">
            <SpeedIcon />
          </div>
          <span class="text-md m-0 w-full text-center">kph</span>
        </div>
      </div>
      <div class="text-lg font-bold text-primary-300">
        {data[DataType.Distance].value}<span class="text-lg ml-1">km dist</span>
      </div>
    </div>
  </div>
</div>
