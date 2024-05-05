import api from './api.js'

let response = await api.get('/api/get')

export default response.data
