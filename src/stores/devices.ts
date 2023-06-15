import { writable } from 'svelte/store'
import { DeviceType, type Device } from '../types'

type UpdateFn = (device: Device) => Device

export const devices = writable<Array<Device>>([
  {
    type: DeviceType.HeartRate,
    title: 'Heart Rate',
    isConnected: false,
  },
  {
    type: DeviceType.SmartTrainer,
    title: 'Smart trainer',
    isConnected: false,
  },
])

export const updateDevices = (updateFn: UpdateFn) => {
  devices.update((items) => items.map(updateFn))
}
