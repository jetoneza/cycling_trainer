export type BasicObject = {
  [key: string]: any
}

export type Device = {
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

export enum DeviceType {
  HeartRate = 'heart_rate',
  SmartTrainer = 'smart_trainer',
  Generic = 'generic',
}

export enum Page {
  Main = 'main',
  Devices = 'devices',
  Workout = 'workout',
  Activities = 'activities',
}

export enum DispatchMessage {
  PageChange = 'pagechange',
}

export enum DataType {
  Distance = 'distance',
  Speed = 'speed',
  HeartRate = 'heart-rate',
  Power = 'power',
  TargetPower = 'target-power',
  Cadence = 'cadence',
  TargetCadence = 'target-cadence',
  ElapsedTime = 'elapsed-time',
  IntervalTime = 'invertal-time',
}

export type Activity = {
  id: string
  name: string
  description: string
  ftp: number
  workouts: Array<Workout>
}

export type Workout = {
  workoutType: WorkoutType
  status: WorkoutStatus
  duration: number
  cadence: number
  powerLow: number
  powerHigh: number
  powerSteady: number
}

export enum WorkoutType {
  Warmup = 'Warmup',
  SteadyState = 'SteadyState',
  Cooldown = 'Cooldown',
}

export enum WorkoutStatus {
  Active = 'active',
  Done = 'done',
}

export enum SessionStatus {
  Started = 'started',
  Paused = 'paused',
  Stopped = 'stopped',
}

export type SessionData = {
  status: SessionStatus
  cadenceData: Array<number>
  powerData: Array<number>
  speedData: Array<number>
  heartRateData: Array<number>
  totalDistance: number
}
