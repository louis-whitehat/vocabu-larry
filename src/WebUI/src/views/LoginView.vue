<template>
  <div>
    <h1>Welcome to Vocabu-Larry ;-)</h1>

    <br />

    <table style="margin: 0 auto">
      <tr>
        <td class="label">
          <label style="font-weight: bold">Who are you? </label>
        </td>
        <td class="value">
          <select v-model="name">
            <option v-for="item in users" v-bind:key="item.name" :value="item.name">
              {{ item.name }}
            </option>
          </select>
        </td>
      </tr>
      <tr v-if="name">
        <td class="label">
          <label style="font-weight: bold">Choose a dictionary: </label>
        </td>
        <td class="value">
          <select v-model="dictionary">
            <option v-for="item in dictionaries" v-bind:key="item" :value="item">
              {{ item }}
            </option>
          </select>
        </td>
      </tr>
    </table>
  </div>
</template>

<script>
import api from '@/api.js'

export default {
  name: 'LoginView',
  data() {
    return {
      name: null,
      dictionary: null,
      users: null
    }
  },
  computed: {
    dictionaries() {
      return this.users.find((x) => x.name === this.name).dictionaries
    }
  },
  watch: {
    dictionary() {
      this.$router.push({
        name: 'exam',
        params: { user: this.name, dictionary: this.dictionary }
      })
    }
  },
  async created() {
    let response = await api.get('/api/users')
    this.users = response.data
  }
}
</script>

<style scoped>
.label {
  width: 200px;
  text-align: left;
}
.value {
  width: 200px;
  text-align: left;
}
</style>
