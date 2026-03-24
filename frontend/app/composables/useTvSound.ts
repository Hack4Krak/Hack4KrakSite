export function useTvSound() {
  let audioCtx: AudioContext | null = null

  function getContext() {
    if (!audioCtx) {
      audioCtx = new AudioContext()
    }
    return audioCtx
  }

  function playTone(frequency: number, duration: number, type: OscillatorType = 'square') {
    const ctx = getContext()
    const osc = ctx.createOscillator()
    const gain = ctx.createGain()

    osc.type = type
    osc.frequency.setValueAtTime(frequency, ctx.currentTime)
    gain.gain.setValueAtTime(0.15, ctx.currentTime)
    gain.gain.exponentialRampToValueAtTime(0.001, ctx.currentTime + duration)

    osc.connect(gain)
    gain.connect(ctx.destination)
    osc.start()
    osc.stop(ctx.currentTime + duration)
  }

  function playFirstSolve() {
    playTone(440, 0.15)
    setTimeout(playTone, 100, 554, 0.15)
    setTimeout(playTone, 200, 659, 0.3)
  }

  function playAllSolved() {
    playTone(523, 0.2, 'square')
    setTimeout(playTone, 150, 659, 0.2, 'square')
    setTimeout(playTone, 300, 784, 0.2, 'square')
    setTimeout(playTone, 450, 1047, 0.5, 'square')
  }

  return { playFirstSolve, playAllSolved }
}
