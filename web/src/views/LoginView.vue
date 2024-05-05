<template>
  <div>
    <h1>Welcome to Vocabu-Larry ;-)</h1>
    <br />
    <label style="font-weight: bold">Dictionary: </label>
    <select v-model="dictionary">
      <option v-for="item in dictionaries" v-bind:key="item.name" :value="item.name">
        {{ item.name }}
      </option>
    </select>
  </div>
</template>

<script>
import store from '../store.js'

export default {
  name: 'LoginView',
  data() {
    return {
      dictionary: null
    }
  },
  computed: {
    dictionaries() {
      return store.dictionaries
    }
  },
  watch: {
    dictionary() {
      this.$router.push({ name: 'exam', params: { dictionaryName: this.dictionary } })
    }
  }
}
</script>
