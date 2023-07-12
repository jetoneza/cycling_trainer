import { listen, type Event as TauriEvent } from '@tauri-apps/api/event'
import { devicesStore } from '../stores/devices'
import { DeviceType, type BasicObject } from '../types'

const setup = () => {
  listen('hrm-notification', (event: TauriEvent<any>) => {
    const { payload } = event

    notifyDevice(payload, DeviceType.HeartRate)
  })

  listen('indoor-bike-notification', (event: TauriEvent<any>) => {
    const { payload } = event

    notifyDevice(payload, DeviceType.SmartTrainer)
  })
}

const notifyDevice = (data: BasicObject, type: DeviceType) => {
  devicesStore.update((map) => {
    const device = map[type]

    const updatedDevice = {
      ...device,
    }

    updatedDevice.bleDevice.data = data

    map[type] = updatedDevice

    return map
  })
}

export default {
  setup,
}
