<script lang="ts">
import { onMount } from 'svelte'
import Chart, {
  type ChartConfiguration,
  type ChartTypeRegistry,
} from 'chart.js/auto'

// Types
import { DataType, type Activity, type BasicObject } from '../../../types'
import type { Writable } from 'svelte/store'

// Utils
import { getActivityDuration } from '../../../utils/time'
import { ZONE_COLORS, getDefaultChartOptions } from '../../../utils/zones'

const CHART_MAX = 210

// Props
export let activity: Activity
export let elapsedTime: Writable<number>
export let devices: BasicObject

let chartCanvas: HTMLCanvasElement
let chart: Chart<keyof ChartTypeRegistry, any[], any>

const activityDuration = getActivityDuration(activity)

onMount(() => {
  const chartContext = chartCanvas.getContext('2d') as CanvasRenderingContext2D

  const initialData = Array(activityDuration).fill(0)

  const labels = initialData

  const options = getDefaultChartOptions(
    'line',
    labels,
    ZONE_COLORS.red,
    CHART_MAX
  ) as ChartConfiguration<keyof ChartTypeRegistry, any[], any>

  chart = new Chart(chartContext, options)
})

const addData = () => {
  const heartRate = devices[DataType.HeartRate].value

  chart.data.datasets[0].data.push(heartRate)
  chart.update()
}

$: {
  if ($elapsedTime > 0) {
    addData()
  }
}
</script>

<canvas bind:this="{chartCanvas}" id="heartRateChart"></canvas>
