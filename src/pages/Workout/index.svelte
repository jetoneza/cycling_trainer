<script lang="ts">
import { DataType } from '../../types'

const data = {
  [DataType.Distance]: {
    value: 19.1,
    unit: 'km',
  },
  [DataType.Speed]: {
    value: 32,
    unit: 'kph',
  },
  [DataType.HeartRate]: {
    value: 120,
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
    value: 100,
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

const distanceAndSpeed = [DataType.Distance, DataType.Speed]
const mainData = [
  DataType.HeartRate,
  DataType.Power,
  DataType.Cadence,
  DataType.IntervalTime,
  DataType.TargetPower,
  DataType.TargetCadence,
]
</script>

<div class="workout-page flex p-4 justify-between">
  <div class="distance-and-speed flex-col space-y-4 basis-1/6 bg-blue-100">
    {#each distanceAndSpeed as type}
      {#if data[type]}
        <div class="item text-center">
          <div class="value font-bold text-2xl">
            {data[type].value}
            {data[type].unit}
          </div>
          <div class="text-sm">{type}</div>
        </div>
      {/if}
    {/each}
  </div>

  <div class="main-data grid grid-cols-3 bg-pink-100 basis-4/6">
    {#each mainData as type}
      {#if data[type]}
        <div class="item text-center">
          <div class="value font-bold text-2xl">
            {data[type].value}
            {data[type].unit}
          </div>
          <div class="text-sm">{type}</div>
        </div>
      {/if}
    {/each}
  </div>

  {#if data[DataType.ElapsedTime]}
    <div class="item text-center bg-green-100 basis-1/6">
      <div class="value font-bold text-2xl">
        {data[DataType.ElapsedTime].value}
        {data[DataType.ElapsedTime].unit}
      </div>
      <div class="text-sm">{DataType.ElapsedTime}</div>
    </div>
  {/if}
</div>
