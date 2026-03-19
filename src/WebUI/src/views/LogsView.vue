<template>
  <div class="page-shell">
    <section class="panel-card">
      <h1 class="page-title">Backend logs</h1>
      <p class="page-copy">Inspect daily request failures and login events without leaving the app.</p>

      <div v-if="errorMessage" class="error-message">{{ errorMessage }}</div>

      <div v-if="files.length" class="log-picker">
        <label for="log-file" class="field-label">Log file</label>
        <select id="log-file" :value="selectedFile" @change="changeFile($event.target.value)">
          <option v-for="file in files" :key="file" :value="file">
            {{ file }}
          </option>
        </select>
      </div>

      <div v-if="files.length">
        <pre>{{ content }}</pre>
      </div>

      <div v-else class="muted-note">No log files found.</div>

      <div class="actions-row">
        <router-link :to="{ name: 'login' }" class="secondary-action">
          Home
        </router-link>
      </div>
    </section>
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
  .error-message {
    margin-bottom: 20px;
    color: #a02222;
    font-weight: 700;
  }

  .log-picker {
    display: flex;
    align-items: center;
    gap: 14px;
    margin-bottom: 20px;
  }

  pre {
    text-align: left;
    white-space: pre-wrap;
    border: 1px solid rgba(54, 105, 170, 0.2);
    border-radius: 18px;
    background: rgba(255, 255, 255, 0.7);
    padding: 16px;
    max-height: 70vh;
    overflow: auto;
  }

  @media (max-width: 640px) {
    .log-picker {
      align-items: stretch;
      flex-direction: column;
    }
  }
</style>