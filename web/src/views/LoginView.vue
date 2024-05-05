<template>
  <div>
    <h1 style="text-align: center">Welcome to Vocabu-Larry ;-)</h1>
    <br />
    <label style="font-weight: bold">Dictionary: </label>
    <select v-model="dictionaryFile">
      <option v-for="item in dictionaryFiles" v-bind:key="item.name" :value="item">{{ item.name }}</option>
    </select>
    <br /><br />
    <ExamView :dictionary-file="dictionaryFile" />
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
      dictionaryFile: null
    }
  },
  computed: {
    dictionaryFiles () { return store.dictionaries }
  },
  created() {
    this.dictionaryFile = store.dictionaries[0]
  }
}
</script>

