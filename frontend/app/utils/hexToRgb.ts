export function rgbToHex(rgb: string): string {
  const rgbValues = rgb.replace(/rgba?\(|\)/g, '').split(',').map(v => Number.parseInt(v.trim(), 10))
  return `#${rgbValues.map(v => v.toString(16).padStart(2, '0')).join('')}`
}

export function hexToRgb(hex: string): string {
  return hex.replace('#', '').match(/.{1,2}/g)?.map(v => Number.parseInt(v, 16)).join(', ') || '255, 255, 255'
}
