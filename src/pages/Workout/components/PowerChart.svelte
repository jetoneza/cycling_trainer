<script lang="ts">
import { onMount } from 'svelte'
import Chart, { type ScriptableChartContext } from 'chart.js/auto'

// Types
import { DataType, type Activity, type BasicObject } from '../../../types'
import type { Writable } from 'svelte/store'

// Utils
import { getActivityDuration } from '../../../utils/time'
import { getZones } from '../../../utils/zones'

const CHART_MAX_THRESHOLD = 1.2

// Props
export let activity: Activity
export let elapsedTime: Writable<number>
export let devices: BasicObject

let chartCanvas: HTMLCanvasElement
let chart: Chart<'line', any[], any>
let chartMax = activity.ftp * CHART_MAX_THRESHOLD

const activityDuration = getActivityDuration(activity)

onMount(() => {
  const chartContext: CanvasRenderingContext2D = chartCanvas.getContext('2d')

  const initialData = Array(activityDuration).fill(0)

  const labels = initialData
  const data = initialData

  chart = new Chart(chartContext, {
    type: 'line',
    data: {
      labels,
      datasets: [
        {
          data,
          label: 'Power',
          borderColor: color,
          pointRadius: 0,
          borderWidth: 5,
        },
      ],
    },
    options: {
      animation: {
        duration: 300,
      },
      maintainAspectRatio: false,
      plugins: {
        legend: {
          display: false,
        },
      },
      scales: {
        x: {
          display: false,
        },
        y: {
          display: false,
          max: chartMax,
          min: 0,
        },
      },
    },
  })
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

const addData = (index: number) => {
  const power = devices[DataType.Power].value

  if (power > chartMax) {
    chartMax = power
    chart.options.scales.y.max = power
  }

  chart.data.datasets[0].data[index] = power
  chart.update()
}

$: {
  if ($elapsedTime > 0) {
    addData($elapsedTime - 1)
  }
}
</script>

<canvas bind:this="{chartCanvas}" id="powerChart"></canvas>
