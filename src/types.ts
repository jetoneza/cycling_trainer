export interface BasicObject {
  [key: string]: any
}

export enum DeviceType {
  HeartRate = 'heart_rate',
  SmartTrainer = 'smart_trainer',
  Generic = 'generic',
}

export interface Device {
  type: DeviceType
  title: string
  name?: string
  bleDevice?: {
    id: string
    name: string
    data?: BasicObject
  }
  isConnected: boolean
}
