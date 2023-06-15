<script lang="ts">
import HeartIcon from 'svelte-icons/fa/FaHeartbeat.svelte'
import BikeIcon from 'svelte-icons/md/MdDirectionsBike.svelte'
import { DeviceType, type BasicObject, type Device } from '../../../types'

export let device: Device
export let handleAction: (device: BasicObject) => {}

const getDeviceHRM = (device: Device) => {
  const { bpm, is_sensor_contact_supported, is_sensor_in_contact } =
    device.bleDevice.data

  if (!is_sensor_contact_supported) {
    return bpm
  }

  return is_sensor_in_contact ? bpm : '--'
}
</script>

<button
  class="device relative box-border border p-4 rounded-2xl flex flex-col justify-between {device.isConnected
    ? 'is-connected'
    : ''}"
  on:click="{() => handleAction(device)}"
>
  <div class="icon h-10 w-10 m-2">
    {#if device.type === DeviceType.HeartRate}
      <HeartIcon />
    {/if}
    {#if device.type === DeviceType.SmartTrainer}
      <BikeIcon />
    {/if}
  </div>

  {#if device.isConnected}
    <span class="connected-indicator absolute left-4 flex h-2 w-2">
      <span
        class="animate-ping absolute inline-flex h-full w-full rounded-full bg-primary-300 opacity-75"
      ></span>
      <span class="relative inline-flex rounded-full h-2 w-2 bg-primary-400"
      ></span>
    </span>
  {/if}

  <div class="name text-left">
    {#if device.isConnected && device.type === DeviceType.HeartRate && device.bleDevice.data}
      <div class="text-white font-bold">
        <span class="text-5xl">{getDeviceHRM(device)}</span>
        <span class="text-lg">bpm</span>
      </div>
    {/if}

    {#if device.isConnected && device.type === DeviceType.SmartTrainer && device.bleDevice.data}
      {#if device.bleDevice.data.power !== null}
        <div class="text-white font-bold">
          <span class="text-5xl">{device.bleDevice.data.power}</span>
          <span class="text-lg">watt</span>
        </div>
      {/if}
      <div class="flex flex-row space-x-4">
        {#if device.bleDevice.data.speed !== null}
          <div class="text-white font-bold">
            <span class="text-sm">{device.bleDevice.data.speed}</span>
            <span class="text-sm">kph</span>
          </div>
        {/if}

        {#if device.bleDevice.data.cadence !== null}
          <div class="text-white font-bold">
            <span class="text-sm">{device.bleDevice.data.cadence}</span>
            <span class="text-sm">rpm</span>
          </div>
        {/if}
      </div>
    {/if}

    <div class="title font-bold flex space-x-2">
      <div>{device.title}</div>
    </div>

    <div class="device-name font-semibold text-secondary-100 text-xs">
      {device.isConnected
        ? `${device.bleDevice?.name || ''} connected`
        : 'Click to pair'}
    </div>
  </div>
</button>
