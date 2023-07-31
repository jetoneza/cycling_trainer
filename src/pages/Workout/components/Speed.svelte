<script lang="ts">
// Icons
import SpeedIcon from 'svelte-icons/io/IoMdSpeedometer.svelte'

// Types
import { DataType, type BasicObject } from '../../../types'

// Props
export let devices: BasicObject

const formatDistance = (distance: number): number | string =>
  distance < 1000 ? distance : (distance / 1000).toFixed(1)

const formatDistanceUnit = (distance: number): string => {
  return `${distance < 1000 ? 'm' : 'km'} dist`
}
</script>

<div class="speed absolute bottom-12 right-12 text-right">
  <div class="rounded-lg bg-secondary-200 px-4 py-2">
    <div class="flex justify-end space-x-1 font-bold text-white">
      <div class="text-6xl">
        {devices[DataType.Speed].value}
      </div>
      <div class="flex flex-col">
        <div class="icon h-10 w-10">
          <SpeedIcon />
        </div>
        <span class="text-md m-0 w-full text-center">kph</span>
      </div>
    </div>
    <div class="text-lg font-bold text-primary-300">
      {formatDistance(devices[DataType.Distance].value)}
      <span class="ml-1 text-lg">
        {formatDistanceUnit(devices[DataType.Distance].value)}
      </span>
    </div>
  </div>
</div>
