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

const port = 8101
const httpsPort = 8102

if (process.env.NODE_ENV === 'production') {
  app.use(express.static('public'))
}

app.use(cors())

app.get('/api/get', async (req, res) => {
  var users = fs.readdirSync('../../dictionaries')
  const store = {
    users: users.map((user) => {
      return {
        name: user,
        dictionaries: fs.readdirSync(path.join('../../dictionaries', user)).map((file) => {
          return {
            name: path.parse(file).name,
            content: fs.readFileSync(path.join('../../dictionaries', user, file), 'utf-8')
          }
        })
      }
    })
  }
  res.json(store)
})

app.listen(port, () => {
  console.log(`Listening at http://localhost:${port}`)
})

server.listen(httpsPort, function () {
  console.log(`Listening at https://localhost:${httpsPort}`)
})
