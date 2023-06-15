import { listen, type Event as TauriEvent } from '@tauri-apps/api/event'
import { devices } from '../stores/devices'
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
  devices.update((items) =>
    items.map((device) => {
      if (device.type == type) {
        const updatedDevice = {
          ...device,
        }

        updatedDevice.bleDevice.data = data

        return updatedDevice
      }

      return device
    })
  )
}

export default {
  setup,
}
