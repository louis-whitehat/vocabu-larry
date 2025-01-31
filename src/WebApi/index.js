const express = require('express')
const cors = require('cors')
const fs = require('fs')
const asyncFs = require('fs').promises
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
app.use(express.json())

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
  const file = path.join(home, 'dictionaries', req.query.user, req.query.dictionary + '.txt')
  const content = await asyncFs.readFile(file, 'utf-8')
  res.send(content)
})

function formatDate(date) {
  const year = date.getFullYear()
  let month = date.getMonth() + 1
  let day = date.getDate()

  if (day < 10) {
    day = '0' + day
  }
  if (month < 10) {
    month = '0' + month
  }

  return `${year}-${month}-${day}`
}

app.post('/api/score', async (req, res) => {
  const { user, isCorrect, dictionary } = req.body

  const file = path.join(home, 'score-' + user + '.json')

  let data = null
  if (fs.existsSync(file)) {
    const content = await asyncFs.readFile(file, 'utf-8')
    data = JSON.parse(content)
  } else {
    data = {}
  }

  const date = formatDate(new Date())

  if (!data[date]) {
    data[date] = {}
  }

  if (!data[date][dictionary]) {
    data[date][dictionary] = {
      total: 0,
      correct: 0
    }
  }

  data[date][dictionary].total += 1
  if (isCorrect) {
    data[date][dictionary].correct += 1
  }

  const content = JSON.stringify(data)
  await asyncFs.writeFile(file, content, 'utf-8')

  res.send('ok')
})

app.get('/api/score', async (req, res) => {
  const file = path.join(home, 'score-' + req.query.user + '.json')
  if (fs.existsSync(file)) {
    const content = await asyncFs.readFile(file, 'utf-8')
    res.json(JSON.parse(content))
  } else {
    res.json({})
  }
})

app.listen(port, () => {
  console.log(`Listening at http://localhost:${port}`)
})

server.listen(httpsPort, function () {
  console.log(`Listening at https://localhost:${httpsPort}`)
})
