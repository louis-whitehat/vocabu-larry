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
import frenchDictionary from '../../../dictionaries/french.txt?raw'
import englishDictionary from '../../../dictionaries/english.txt?raw'
import loonyEnglishDictionary from '../../../dictionaries/loony-english.txt?raw'

export default {
  name: 'LoginView',
  components: { ExamView },
  data() {
    return {
      dictionaryFile: null,
      dictionaryFiles: [
        {
          name: 'french',
          content: frenchDictionary
        },
        {
          name: 'english',
          content: englishDictionary
        },
        {
          name: 'loony english',
          content: loonyEnglishDictionary
        }
      ]
    }
  },
  created() {
    this.dictionaryFile = this.dictionaryFiles[0]
  }
}
</script>

