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

const CHART_MAX_THRESHOLD = 1.2
const CHART_PADDING = 5

// Props
export let activity: Activity
export let elapsedTime: Writable<number>
export let devices: BasicObject

let chartCanvas: HTMLCanvasElement
let chart: Chart<keyof ChartTypeRegistry, any[], any>
let chartMax = activity.ftp * CHART_MAX_THRESHOLD

const activityDuration = getActivityDuration(activity)

onMount(() => {
  const chartContext: CanvasRenderingContext2D = chartCanvas.getContext('2d')

  const initialData = Array(activityDuration).fill(0)

  const labels = initialData

  const options = getDefaultChartOptions(
    labels,
    ZONE_COLORS.red,
    chartMax
  ) as ChartConfiguration<keyof ChartTypeRegistry, any[], any>

  chart = new Chart(chartContext, options)
})

const addData = () => {
  const heartRate = devices[DataType.HeartRate].value

  if (heartRate > chartMax) {
    chartMax = heartRate
    chart.options.scales.y.max = heartRate + CHART_PADDING
  }

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
