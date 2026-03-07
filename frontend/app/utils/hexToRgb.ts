const rgbaPattern = /rgba?\(|\)/g
const hexPairPattern = /.{1,2}/g

export function rgbToHex(rgb: string): string {
  const rgbValues = rgb.replace(rgbaPattern, '').split(',').map(v => Number.parseInt(v.trim(), 10))
  return `#${rgbValues.map(v => v.toString(16).padStart(2, '0')).join('')}`
}

export function hexToRgb(hex: string): string {
  return hex.replace('#', '').match(hexPairPattern)?.map(v => Number.parseInt(v, 16)).join(', ') || '255, 255, 255'
}
