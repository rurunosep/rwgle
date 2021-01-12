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

// TODO: move this out
export const loadTextureImage = (gl, texture, sourceUrl) => {
  let image = new Image()
  image.src = sourceUrl
  image.addEventListener('load', () => {
    gl.bindTexture(gl.TEXTURE_2D, texture)
    gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, image)
  })
}
