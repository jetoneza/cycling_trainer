import { writable, get } from 'svelte/store'

export enum TimerStatus {
  Started = 'started',
  Paused = 'paused',
  Stopped = 'stopped',
}

export const useTimer = () => {
  const elapsedTime = writable(0)
  const intervalTime = writable(0)
  const status = writable(TimerStatus.Stopped)

  let timer: NodeJS.Timer

  const start = () => {
    status.set(TimerStatus.Started)

    timer = setInterval(() => {
      elapsedTime.update((time) => time + 1)
      intervalTime.update((time) => time + 1)
    }, 1000)
  }

  const pause = () => {
    clearInterval(timer)

    status.set(TimerStatus.Paused)
  }

  const stop = () => {
    clearInterval(timer)

    status.set(TimerStatus.Stopped)
    elapsedTime.set(0)
    intervalTime.set(0)
  }

  const getStatus = () => get(status)
  const resetInterval = () => intervalTime.set(0)

  return {
    elapsedTime,
    intervalTime,
    getStatus,
    start,
    pause,
    stop,
    resetInterval,
  }
}
