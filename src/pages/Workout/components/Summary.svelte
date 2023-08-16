<script lang="ts">
import Chart, {
  type ChartTypeRegistry,
  type ChartConfiguration,
  type ScriptableContext,
} from 'chart.js/auto'
import { onMount } from 'svelte'

// Utils
import {
  ZONE_COLORS,
  getDefaultChartOptions,
  getZones,
} from '../../../utils/zones'
import { average } from '../../../utils/common'

// Types
import type { SessionData } from '../../../types'

export let sessionData: SessionData
export let onBackClick: () => void
export let onSaveClick: () => void

let cadenceChartCanvas: HTMLCanvasElement
let hrmChartCanvas: HTMLCanvasElement
let powerChartCanvas: HTMLCanvasElement
let speedChartCanvas: HTMLCanvasElement

const limits = {
  hrm: {
    max: 0,
    ave: 0,
  },
  power: {
    max: 0,
    ave: 0,
  },
  cadence: {
    max: 0,
    ave: 0,
  },
  speed: {
    max: 0,
    ave: 0,
  },
}

onMount(() => {
  generateLineChart('heartRateData', ZONE_COLORS.red)
  generateLineChart('speedData', ZONE_COLORS.green)
  generateLineChart('cadenceData', ZONE_COLORS.blue)
  generatePowerChart()
})

const generateLineChart = (dataName: string, color: string) => {
  const LINE_CHART_CANVAS = {
    heartRateData: {
      key: 'hrm',
      canvas: hrmChartCanvas,
    },
    cadenceData: {
      key: 'cadence',
      canvas: cadenceChartCanvas,
    },
    speedData: {
      key: 'speed',
      canvas: speedChartCanvas,
    },
  }

  const { key, canvas } = LINE_CHART_CANVAS[dataName]

  const context: CanvasRenderingContext2D = canvas.getContext('2d')

  const data = sessionData[dataName]

  const max = Math.max(...data)
  const min = Math.min(...data)

  limits[key].max = max
  limits[key].ave = Math.floor(average(data))

  const options = getDefaultChartOptions(
    'line',
    data,
    color,
    max
  ) as ChartConfiguration<keyof ChartTypeRegistry, any[], any>

  options.data.datasets[0].data = data

  const yAxis = {
    display: true,
    max,
    min,
  }

  options.options.scales.y = yAxis

  new Chart(context, options)
}

const generatePowerChart = () => {
  const context: CanvasRenderingContext2D = powerChartCanvas.getContext('2d')

  const { powerData } = sessionData as SessionData

  const max = Math.max(...powerData)
  const min = Math.min(...powerData)

  limits.power.max = max
  limits.power.ave = Math.floor(average(powerData))

  const color = getColor(max, 164)

  const options = getDefaultChartOptions(
    'bar',
    powerData,
    color,
    max
  ) as ChartConfiguration<keyof ChartTypeRegistry, any[], any>

  options.data.datasets[0].data = powerData

  const yAxis = {
    display: true,
    max,
    min,
  }

  options.options.scales.y = yAxis

  new Chart(context, options)
}

const getColor =
  (chartMax: number, ftp: number) => (context: ScriptableContext<'bar'>) => {
    if (!context.chart.chartArea) {
      return
    }

    const index = context.dataIndex
    const value = context.dataset.data[index] as number

    const zones = getZones(ftp, chartMax)

    let color = zones[0].color

    zones.forEach((zone) => {
      let powerThreshold = value / chartMax

      if (powerThreshold >= zone.threshold) {
        color = zone.color
      }
    })

    return color
  }
</script>

<div class="summary">
  <div class="text-center text-2xl font-bold">Session Summary</div>

  <div class="mt-10 flex flex-col space-y-4 pr-4">
    <div class="heart-rate-chart flex h-24">
      <div class="limit flex w-32 flex-col p-4 text-center text-sm">
        <span class="font-bold"> Heart Rate </span>
        <span>Max {limits.hrm.max}</span>
        <span>Avg {limits.hrm.ave}</span>
      </div>
      <div class="wrapper flex-auto">
        <canvas bind:this="{hrmChartCanvas}"></canvas>
      </div>
    </div>

    <div class="power-chart flex h-24">
      <div class="limit flex w-32 flex-col p-4 text-center text-sm">
        <span class="font-bold"> Speed </span>
        <span>Max {limits.speed.max}</span>
        <span>Avg {limits.speed.ave}</span>
      </div>
      <div class="wrapper flex-auto">
        <canvas bind:this="{speedChartCanvas}"></canvas>
      </div>
    </div>

    <div class="power-chart flex h-24">
      <div class="limit flex w-32 flex-col p-4 text-center text-sm">
        <span class="font-bold"> Cadence </span>
        <span>Max {limits.cadence.max}</span>
        <span>Avg {limits.cadence.ave}</span>
      </div>
      <div class="wrapper flex-auto">
        <canvas bind:this="{cadenceChartCanvas}"></canvas>
      </div>
    </div>

    <div class="power-chart flex h-24">
      <div class="limit flex w-32 flex-col p-4 text-center text-sm">
        <span class="font-bold"> Power </span>
        <span>Max {limits.power.max}</span>
        <span>Avg {limits.power.ave}</span>
      </div>
      <div class="wrapper flex-auto">
        <canvas bind:this="{powerChartCanvas}"></canvas>
      </div>
    </div>
  </div>

  <div class="actions absolute bottom-12 flex w-full justify-center space-x-6">
    <button class="btn secondary w-1/6 text-2xl" on:click="{onBackClick}">
      Back
    </button>
    <button class="btn w-1/6 text-2xl" on:click="{onSaveClick}"> Save </button>
  </div>
</div>
