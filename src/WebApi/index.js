const express = require('express')
const cors = require('cors')
const fs = require('fs')
const https = require('https')
const path = require('path')

const home = process.env.NODE_ENV === 'production' ? process.env.VOCABULARRY_HOME : '../../'

function load(file) {
  return fs.existsSync(file) ? fs.readFileSync(file) : null
}

const key = load(`${home}/selfsigned.key`)
const cert = load(`${home}/selfsigned.crt`)

const app = express()
const server = https.createServer({ key: key, cert: cert }, app)

const port = 8101
const httpsPort = 8102

if (process.env.NODE_ENV === 'production') {
  app.use(express.static('public'))
}

app.use(cors())

app.get('/api/users', async (req, res) => {
  const dataDir = path.join(home, 'dictionaries')
  var users = fs.readdirSync(dataDir)
  const store = users.map((user) => {
    return {
      name: user,
      dictionaries: fs.readdirSync(path.join(dataDir, user)).map((file) => path.parse(file).name)
    }
  })
  res.json(store)
})

app.get('/api/dictionary', async (req, res) => {
  const file = path.join(home, 'dictionaries', req.query.user, req.query.dictionary + ".txt")
  const content = fs.readFileSync(file, 'utf-8')
  res.send(content)
})

app.listen(port, () => {
  console.log(`Listening at http://localhost:${port}`)
})

server.listen(httpsPort, function () {
  console.log(`Listening at https://localhost:${httpsPort}`)
})
