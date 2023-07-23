import type { ScriptableChartContext } from 'chart.js'
import type { BasicObject } from '../types'

export const ZONE_COLORS = {
  red: 'rgb(255, 99, 132)',
  orange: 'rgb(255, 159, 64)',
  yellow: 'rgb(255, 205, 86)',
  green: 'rgb(75, 192, 192)',
  blue: 'rgb(54, 162, 235)',
  grey: 'rgb(201, 203, 207)',
}

export const ZONES: BasicObject = {
  Z1: {
    threshold: 0,
    color: ZONE_COLORS.grey,
  },
  Z2: {
    threshold: 0.6,
    color: ZONE_COLORS.blue,
  },
  Z3: {
    threshold: 0.76,
    color: ZONE_COLORS.green,
  },
  Z4: {
    threshold: 0.9,
    color: ZONE_COLORS.yellow,
  },
  Z5: {
    name: 'z5',
    threshold: 1.05,
    color: ZONE_COLORS.orange,
  },
  Z6: {
    threshold: 1.18,
    color: ZONE_COLORS.red,
  },
}

export const getDefaultChartOptions = (
  type: string,
  labels: Array<any>,
  color: ((context: ScriptableChartContext) => CanvasGradient) | string,
  height: number
) => ({
  type,
  data: {
    labels,
    datasets: [
      {
        data: [],
        backgroundColor: color,
        borderColor: color,
        pointRadius: 0,
        borderWidth: 1,
        barPercentage: 0.5,
        barThickness: 6,
        maxBarThickness: 8,
        minBarLength: 2,
      },
    ],
  },
  options: {
    animation: {
      duration: 0,
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
        max: height,
        min: 0,
      },
    },
  },
})

export const getZones = (ftp: number, maxPower: number) =>
  Object.keys(ZONES).map((key) => {
    const zoneValue = Math.floor(ftp * ZONES[key].threshold)
    const threshold = zoneValue / maxPower

    return {
      threshold,
      color: ZONES[key].color,
    }
  })
