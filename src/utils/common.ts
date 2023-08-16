export const average = (numbers: Array<number>): number => {
  const sum = numbers.reduce((acc, curr) => acc + curr, 0)
  return sum / numbers.length
}
