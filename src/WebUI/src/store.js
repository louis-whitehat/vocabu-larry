import api from './api.js'

let response = await api.get('/api/get')
console.log(response)

export default response.data
