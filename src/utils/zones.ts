import type { BasicObject } from '../types'

const COLORS = {
  red: 'rgb(255, 99, 132)',
  orange: 'rgb(255, 159, 64)',
  yellow: 'rgb(255, 205, 86)',
  green: 'rgb(75, 192, 192)',
  blue: 'rgb(54, 162, 235)',
  purple: 'rgb(153, 102, 255)',
  grey: 'rgb(201, 203, 207)',
}

export const ZONES: BasicObject = {
  Z1: {
    threshold: 0,
    color: COLORS.grey,
  },
  Z2: {
    threshold: 0.6,
    color: COLORS.blue,
  },
  Z3: {
    threshold: 0.76,
    color: COLORS.green,
  },
  Z4: {
    threshold: 0.9,
    color: COLORS.yellow,
  },
  Z5: {
    name: 'z5',
    threshold: 1.05,
    color: COLORS.orange,
  },
  Z6: {
    threshold: 1.18,
    color: COLORS.red,
  },
}

export const getZones = (ftp: number, maxPower: number) =>
  Object.keys(ZONES).map((key) => {
    const zoneValue = Math.floor(ftp * ZONES[key].threshold)
    const threshold = zoneValue / maxPower

    return {
      threshold,
      color: ZONES[key].color,
    }
  })
