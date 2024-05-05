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
            <option v-for="item in names" v-bind:key="item.name" :value="item.name">
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
            <option v-for="item in dictionaries" v-bind:key="item.name" :value="item.name">
              {{ item.name }}
            </option>
          </select>
        </td>
      </tr>
    </table>
  </div>
</template>

<script>
import store from '../store.js'

export default {
  name: 'LoginView',
  data() {
    return {
      name: null,
      dictionary: null
    }
  },
  computed: {
    names() {
      return store.users
    },
    dictionaries() {
      return store.users.find((x) => x.name === this.name).dictionaries
    }
  },
  watch: {
    dictionary() {
      this.$router.push({
        name: 'exam',
        params: { name: this.name, dictionaryName: this.dictionary }
      })
    }
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
