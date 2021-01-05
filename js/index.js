import('../wasm/').then((lib) => {
  const canvas = document.getElementById('canvas')

  let engine
  try {
    engine = new lib.RustWebGLEngine(canvas)
  } catch (e) {
    console.log(e)
  }

  const render = () => {
    engine.render()
    window.requestAnimationFrame(render)
  }

  window.requestAnimationFrame(render)
})
