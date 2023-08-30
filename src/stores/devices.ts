import { writable } from 'svelte/store'
import { DeviceType, type Device } from '../types'

type UpdateFn = (map: DevicesMap) => DevicesMap

type DevicesMap = {
  [DeviceType.HeartRate]: Device
  [DeviceType.SmartTrainer]: Device
}

export const devicesStore = writable<DevicesMap>({
  [DeviceType.HeartRate]: {
    type: DeviceType.HeartRate,
    title: 'Heart Rate',
    isConnected: false,
  },
  [DeviceType.SmartTrainer]: {
    type: DeviceType.SmartTrainer,
    title: 'Smart trainer',
    isConnected: false,
  },
})

export const updateDevices = (updateFn: UpdateFn) => {
  devicesStore.update((map) => updateFn(map))
}
