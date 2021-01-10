import('../wasm/').then(({ RustWebGLEngine }) => {
  const canvas = document.getElementById('canvas')

  try {
    let engine = new RustWebGLEngine(canvas)

    const render = () => {
      engine.render()
      window.requestAnimationFrame(render)
    }

    window.requestAnimationFrame(render)
  } catch (e) {
    console.log(e)
  }
})
