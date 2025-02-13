<template>
  <div>
    <table style="margin: 0 auto" v-if="scores">
      <tr>
        <th class="label">Date</th>
        <th class="label">Dictionary</th>
        <th class="value">Correct</th>
        <th class="value">Total</th>
        <th class="value">Pass Rate</th>
      </tr>
      <template v-for="date in sortedDates" :key="date">
        <tr v-for="(stats, dictionary) in scores[date]" :key="dictionary">
          <td class="label">{{ date }}</td>
          <td class="label">{{ dictionary }}</td>
          <td class="value">{{ stats.correct }}</td>
          <td class="value">{{ stats.total }}</td>
          <td class="value">{{ (stats.correct / stats.total * 100).toFixed(2) }}</td>
        </tr>
      </template>
    </table>

    <div>
      <br />
      <router-link style="margin-left: 20px" :to="{ name: 'login' }">
        <button>Home</button>
      </router-link>
    </div>
  </div>
</template>

<script setup>
  import { ref, computed, onMounted } from 'vue'
  import { useRoute } from 'vue-router'
  import api from '@/api.js'

  const route = useRoute()
  const scores = ref(null)

  const sortedDates = computed(() => {
    if (!scores.value) return []
    return Object.keys(scores.value).sort((a, b) => b.localeCompare(a))
  })

  onMounted(async () => {
    try {
      const response = await api.get('/api/score', {
        params: { user: route.params.user }
      })
      scores.value = response.data
    } catch (error) {
      console.error('Error fetching scores:', error)
    }
  })
</script>

<style scoped>
  th {
    font-weight: bold;
  }
  .label {
    width: 100px;
    text-align: left;
  }
  .value {
    width: 50px;
    text-align: right;
  }
</style>
