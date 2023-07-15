const POWER_JUMP = 5

export const calculateRangePower = (
  ftp: number,
  low: number,
  high: number,
  duration: number,
  elapsedTime: number
) => {
  const lowPower = Math.floor(low * ftp)
  const highPower = Math.floor(high * ftp)

  const range = Math.abs(highPower - lowPower)
  const timeJump = Math.floor(duration / Math.floor(range / POWER_JUMP))
  const accumulator = POWER_JUMP * Math.floor(elapsedTime / timeJump)

  let power = lowPower + accumulator

  if (low > high) {
    power = lowPower - accumulator
  }

  return Math.round(power / POWER_JUMP) * POWER_JUMP
}
