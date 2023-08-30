import { writable } from 'svelte/store'
import { DeviceType, type Device } from '../types'

type UpdateFn = (map: DevicesMap) => DevicesMap

interface DevicesMap {
  [DeviceType.HeartRate]: Device
  [DeviceType.SmartTrainer]: Device
  [DeviceType.Generic]: Device
}

export const devicesStore = writable<DevicesMap>({
  [DeviceType.HeartRate]: {
    type: DeviceType.HeartRate,
    title: 'Heart Rate',
    isConnected: false,
    bleDevice: {
      id: 'hrm',
      name: 'Generic Heart Rate',
    },
  },
  [DeviceType.SmartTrainer]: {
    type: DeviceType.SmartTrainer,
    title: 'Smart trainer',
    isConnected: false,
    bleDevice: {
      id: 'bike',
      name: 'Generic Trainer',
    },
  },
  [DeviceType.Generic]: {
    type: DeviceType.Generic,
    title: 'Generic',
    isConnected: false,
    bleDevice: {
      id: 'generic',
      name: 'Generic Device',
    },
  },
})

export const updateDevices = (updateFn: UpdateFn) => {
  devicesStore.update((map) => updateFn(map))
}
