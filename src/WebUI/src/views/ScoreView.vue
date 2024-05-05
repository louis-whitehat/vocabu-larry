<template>
  <div>
    <table style="margin: 0 auto" v-if="score">
      <tr>
        <th>Date</th>
        <th>Correct</th>
        <th>Total</th>
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
        <button>Finished</button>
      </router-link>
    </div>
  </div>
</template>

<script>
import api from '@/api.js'

export default {
  name: 'ScoresView',
  data() {
    return {
      score: null
    }
  },
  computed: {
    dates() {
      return Object.keys(this.score)
    }
  },
  async created() {
    const response = await api.get('/api/score', {
      params: {
        user: this.$route.params.user
      }
    })
    this.score = response.data
  }
}
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
