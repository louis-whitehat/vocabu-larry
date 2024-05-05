<template>
  <div>
    <h1 style="text-align: center">Welcome to Vocabu-Larry ;-)</h1>
    <br />
    <label style="font-weight: bold">Dictionary: </label>
    <select v-model="dictionary">
      <option v-for="item in dictionaries" v-bind:key="item.name" :value="item.name">{{ item.name }}</option>
    </select>
    <br /><br />
    <ExamView :dictionary-name="dictionary" />
  </div>
</template>

<script>
import ExamView from './ExamView.vue'
import store from '../store.js'

export default {
  name: 'LoginView',
  components: { ExamView },
  data() {
    return {
      dictionary: null
    }
  },
  computed: {
    dictionaries () { return store.dictionaries }
  },
  created() {
    this.dictionary = store.dictionaries[0].name
  }
}
</script>

