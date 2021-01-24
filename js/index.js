import('../wasm/').then(async ({ RustWebGLEngine }) => {
  const canvas = document.getElementById('canvas')
  const leftButton = document.getElementById('left-button')
  const rightButton = document.getElementById('right-button')

  try {
    let engine = await new RustWebGLEngine(canvas)

    leftButton.addEventListener('click', () => {
      engine.rotateCameraLeft()
    })
    rightButton.addEventListener('click', () => {
      engine.rotateCameraRight()
    })

    const render = () => {
      engine.render()
      window.requestAnimationFrame(render)
    }

    window.requestAnimationFrame(render)
  } catch (e) {
    console.log(e)
  }
})

// TODO: Either move this out or just do it all in Rust now
// that I understand it a little better
export const loadTextureImage = (gl, texture, sourceUrl) => {
  let image = new Image()
  image.src = sourceUrl
  image.addEventListener('load', () => {
    gl.bindTexture(gl.TEXTURE_2D, texture)
    gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, image)
  })
}
