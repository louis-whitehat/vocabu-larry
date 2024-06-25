import axios from 'axios'

const api = axios.create({
  baseURL: import.meta.env.MODE === 'production' ? window.location.origin : 'http://localhost:8101'
})

export default api