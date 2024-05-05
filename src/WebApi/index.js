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
const server = https.createServer({ key: key, cert: cert }, app)

const port = 8001
const httpsPort = 8002

if (process.env.NODE_ENV === 'production') {
  app.use(express.static('public'))
}

app.use(cors())

app.get('/api/get', async (req, res) => {
  const store = {
    users: [
      {
        name: 'Louis',
        dictionaries: [
          {
            name: 'French',
            content: fs.readFileSync('../../dictionaries/louis/french.txt', 'utf-8')
          },
          {
            name: 'English',
            content: fs.readFileSync('../../dictionaries/louis/english.txt', 'utf-8')
          }
        ]
      },
      {
        name: 'Leonie',
        dictionaries: [
          {
            name: 'English',
            content: fs.readFileSync('../../dictionaries/leonie/english.txt', 'utf-8')
          }
        ]
      }
    ]
  }
  res.json(store)
})

app.listen(port, () => {
  console.log(`Listening at http://localhost:${port}`)
})

server.listen(httpsPort, function () {
  console.log(`Listening at https://localhost:${httpsPort}`)
})
