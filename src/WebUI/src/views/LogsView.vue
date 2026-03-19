<template>
  <div>
    <h1>Backend Logs</h1>

    <div v-if="errorMessage" style="margin-bottom: 20px; color: red">{{ errorMessage }}</div>

    <div v-if="files.length" style="margin-bottom: 20px">
      <label for="log-file" style="font-weight: bold; margin-right: 10px">Log file</label>
      <select id="log-file" :value="selectedFile" @change="changeFile($event.target.value)">
        <option v-for="file in files" :key="file" :value="file">
          {{ file }}
        </option>
      </select>
    </div>

    <div v-if="files.length">
      <pre>{{ content }}</pre>
    </div>

    <div v-else>No log files found.</div>

    <div style="margin-top: 20px">
      <router-link :to="{ name: 'login' }">
        <button>Home</button>
      </router-link>
    </div>
  </div>
</template>

<script setup>
  import { ref, onMounted } from 'vue'
  import api from '@/api.js'

  const files = ref([])
  const selectedFile = ref(null)
  const content = ref('')
  const errorMessage = ref(null)

  const loadLogs = async (file = null) => {
    try {
      const response = await api.get('/api/logs', {
        params: file ? { file } : {}
      })

      files.value = response.data.files
      selectedFile.value = response.data.selectedFile
      content.value = response.data.content
      errorMessage.value = null
    } catch (error) {
      console.error('Error fetching logs:', error)
      errorMessage.value = 'Failed to fetch logs.'
    }
  }

  const changeFile = async (file) => {
    await loadLogs(file)
  }

  onMounted(async () => {
    await loadLogs()
  })
</script>

<style scoped>
  pre {
    text-align: left;
    white-space: pre-wrap;
    border: 1px solid #ccc;
    padding: 16px;
    max-height: 70vh;
    overflow: auto;
  }
</style>