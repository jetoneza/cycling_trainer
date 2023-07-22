<script lang="ts">
import { onMount } from 'svelte'
import Chart, {
  type ChartConfiguration,
  type ChartTypeRegistry,
  type ScriptableChartContext,
} from 'chart.js/auto'

// Types
import { DataType, type Activity, type BasicObject } from '../../../types'
import type { Writable } from 'svelte/store'

// Utils
import { getActivityDuration } from '../../../utils/time'
import { getDefaultChartOptions, getZones } from '../../../utils/zones'

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
    color,
    chartMax
  ) as ChartConfiguration<keyof ChartTypeRegistry, any[], any>

  chart = new Chart(chartContext, options)
})

const color = (context: ScriptableChartContext) => {
  const chart = context.chart
  const { ctx, chartArea } = chart

  if (!chartArea) {
    return
  }

  const gradient = ctx.createLinearGradient(
    0,
    chartArea.bottom,
    0,
    chartArea.top
  )

  const zones = getZones(activity.ftp, chartMax)

  zones.forEach((zone) => {
    gradient.addColorStop(zone.threshold, zone.color)
  })

  return gradient
}

const addData = () => {
  const power = devices[DataType.Power].value

  if (power > chartMax) {
    chartMax = power
    chart.options.scales.y.max = power + CHART_PADDING
  }

  chart.data.datasets[0].data.push(power)
  chart.update()
}

$: {
  if ($elapsedTime > 0) {
    addData()
  }
}
</script>

<canvas bind:this="{chartCanvas}" id="powerChart"></canvas>
