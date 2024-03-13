export const limitu16 = (s: string) => {
  let val = Number(s.replace(/\D+/, '').slice(0, 5))
  if (val > 65535) val = 65535
  return val
}
export const limitu32 = (s: string) => {
  let val = Number(s.replace(/\D+/, '').slice(0, 10))
  if (val > 4294967295) val = 4294967295
  return val
}
