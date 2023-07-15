<script lang="ts">
// Icons
import HeartIcon from 'svelte-icons/fa/FaHeartbeat.svelte'
import LightningIcon from 'svelte-icons/fa/FaBolt.svelte'
import CadenceIcon from 'svelte-icons/go/GoSync.svelte'

// Types
import { DataType, type BasicObject } from '../../../types'

// Props
export let devices: BasicObject
</script>

<div class="data-view flex space-x-4 rounded-lg bg-secondary-200">
  <div class="column flex flex-col justify-between px-6 py-4 text-center">
    <div class="text-2xl font-bold text-white">Target</div>

    <div class="item">
      <div class="value text-4xl font-bold text-primary-300">
        {devices[DataType.TargetPower].value}<span class="text-2xl">w</span>
      </div>
    </div>

    <!-- TODO: Fix UI if cadence is not visible -->
    {#if !!devices[DataType.TargetCadence] && devices[DataType.TargetCadence].value > 0}
      <div class="text-2xl font-bold text-white">at</div>

      <div class="item">
        <div class="value text-4xl font-bold text-primary-300">
          {devices[DataType.TargetCadence].value}<span class="text-2xl"
            >rpm</span
          >
        </div>
      </div>
    {/if}
  </div>

  <div class="column px-6 pb-4 pt-2">
    <div class="row item text-center">
      <div
        class="value flex w-full justify-center text-9xl font-bold text-white"
      >
        {devices[DataType.Power].value}
        <div class="icon ml-2 mt-11 w-10">
          <LightningIcon />
        </div>
      </div>
    </div>

    <div class="row m-0 flex justify-center space-x-6">
      <div class="item">
        <div class="value flex text-4xl font-bold text-white">
          {devices[DataType.HeartRate].value}
          <div class="icon ml-2 mt-3 h-5 w-5">
            <HeartIcon />
          </div>
        </div>
      </div>

      <div class="item">
        <div class="value flex text-4xl font-bold text-white">
          {devices[DataType.Cadence].value}
          <div class="icon ml-1 mt-3 h-5 w-5">
            <CadenceIcon />
          </div>
        </div>
      </div>
    </div>
  </div>

  <div class="column flex flex-col justify-between px-6 py-4 text-center">
    <div class="item text-white">
      <div class="text-xl font-bold">Elapsed</div>
      <div class="value text-4xl font-bold">
        {devices[DataType.ElapsedTime].value}
      </div>
    </div>
    <div class="item text-white">
      <div class="text-xl font-bold">Interval</div>
      <div class="value text-4xl font-bold">
        {devices[DataType.IntervalTime].value}
      </div>
    </div>
  </div>
</div>
