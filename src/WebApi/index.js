const express = require('express')
const cors = require('cors')
const fs = require('fs')
const https = require('https')
const path = require('path')

const configHome = process.env.VOCABULARRY_CONFIG

function load(file) {
  return fs.existsSync(file) ? fs.readFileSync(file) : null
}

const key = load(`${configHome}/selfsigned.key`)
const cert = load(`${configHome}/selfsigned.crt`)

const app = express()
const server = https.createServer({key: key, cert: cert }, app)

const port = 8001
const httpsPort = 8002

if (process.env.NODE_ENV === 'production') {
  app.use(express.static('public'))
} else {
  console.log('dev')
  app.use(express.static(path.join('dist', 'public')));
}

app.use(cors())

// app.get('/api/crypto', async (req, res) => {

//   res.json(prices)
// })

app.listen(port, () => {
  console.log(`Listening at http://localhost:${port}`)
})

server.listen(httpsPort, function () {
  console.log(`Listening at https://localhost:${httpsPort}`)
})