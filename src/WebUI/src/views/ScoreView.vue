<template>
  <div class="page-shell">
    <section class="panel-card">
      <h1 class="page-title">Score</h1>
      <p class="page-copy">Daily totals for the selected learner across all dictionaries.</p>

      <table class="score-table" v-if="scores">
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

      <p v-else class="muted-note">No score entries yet.</p>

      <div class="actions-row">
        <router-link :to="{ name: 'login' }" class="secondary-action">
          Home
        </router-link>
      </div>
    </section>
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
  .score-table {
    margin: 0;
    width: 100%;
  }

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
