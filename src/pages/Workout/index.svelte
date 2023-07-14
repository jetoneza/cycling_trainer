<script lang="ts">
import { DataType, DeviceType, type Activity } from '../../types'

// Stores
import { devicesStore } from '../../stores/devices'
import { activityStore } from '../../stores/activities'

// Styles
import './styles.css'

// Components
import DataView from './components/DataView.svelte'
import Speed from './components/Speed.svelte'
import WorkoutsList from './components/WorkoutsList.svelte'

let devices = {
  [DataType.Distance]: {
    value: 0,
    unit: 'km',
  },
  [DataType.Speed]: {
    value: 0,
    unit: 'kph',
  },
  [DataType.HeartRate]: {
    value: '--',
    unit: 'bpm',
  },
  [DataType.Power]: {
    value: 0,
    unit: 'watt',
  },
  [DataType.TargetPower]: {
    value: 0,
    unit: 'watt',
  },
  [DataType.Cadence]: {
    value: 0,
    unit: 'rpm',
  },
  [DataType.TargetCadence]: {
    value: 0,
    unit: 'rpm',
  },
  [DataType.IntervalTime]: {
    value: '05:34',
    unit: '',
  },
  [DataType.ElapsedTime]: {
    value: '45:31',
    unit: '',
  },
}

let activity: Activity

devicesStore.subscribe((map) => {
  const hrm = map[DeviceType.HeartRate]
  const smartTrainer = map[DeviceType.SmartTrainer]

  if (hrm && hrm.bleDevice) {
    const { bpm, is_sensor_in_contact } = hrm.bleDevice.data
    devices[DataType.HeartRate].value = is_sensor_in_contact ? bpm : '--'
  }

  if (smartTrainer && smartTrainer.bleDevice) {
    const { cadence, distance, power, speed } = smartTrainer.bleDevice.data

    devices[DataType.Distance].value = distance || 0
    devices[DataType.Speed].value = speed || 0
    devices[DataType.Power].value = power || 0
    devices[DataType.Cadence].value = cadence || 0
  }
})

activityStore.subscribe((value) => (activity = value))
</script>

<div class="workout-page flex justify-between py-4">
  <div class="basis-3/12">
    <WorkoutsList activity="{activity}" />
  </div>

  <div class="main-data">
    <DataView devices="{devices}" />
  </div>

  <div class="basis-3/12">
    <!-- TODO: Add other data here. -->
  </div>

  <Speed devices="{devices}" />
</div>
