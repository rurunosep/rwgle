const express = require('express')
const path = require('path')

const app = express()
app.use(express.static(path.join(__dirname, 'dist')))
express.static.mime.define({ 'application/wasm': ['wasm'] })

const port = process.env.PORT || 8080
app.listen(port, () => console.log(`Server started on port: ${port}`))
