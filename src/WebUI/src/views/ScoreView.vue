<template>
  <div>
    <table style="margin: 0 auto" v-if="score">
      <tr>
        <th class="label">Date</th>
        <th class="value">Correct</th>
        <th class="value">Total</th>
      </tr>
      <tr v-for="date in dates" v-bind:key="date">
        <td class="label">{{ date }}</td>
        <td class="value">{{ score[date].correct }}</td>
        <td class="value">{{ score[date].total }}</td>
      </tr>
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
  const score = ref(null)

  const dates = computed(() => (score.value ? Object.keys(score.value) : []))

  onMounted(async () => {
    try {
      const response = await api.get('/api/score', {
        params: { user: route.params.user }
      })
      score.value = response.data
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
