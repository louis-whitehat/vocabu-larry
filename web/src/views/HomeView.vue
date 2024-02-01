<template>
  <main style="font-size: larger">
    <div><label>Translate:</label> {{ word }}</div>

    <form @submit.prevent="submit">
      <input type="text" v-model="input" />
      <button style="margin-left: 20px">Submit</button>
    </form>

    <div style="margin-top: 50px">
      <img src="../assets/checkmark.png" v-if="answerCorrect === true" />
      <img src="../assets/red-cross.png" v-if="answerCorrect === false" />
    </div>
  </main>
</template>

<script>
import dictionary from '../../../dictionaries/english.txt?raw'

export default {
  name: 'HomeView',
  data() {
    return {
      dictionary: null,
      selected: null,
      input: null,
      answerCorrect: null
    }
  },
  computed: {
    word() {
      return this.dictionary[this.selected][0]
    }
  },
  methods: {
    submit() {
      this.answerCorrect = this.dictionary[this.selected][1] == this.input
      this.selectNextEntry()
    },
    selectNextEntry() {
      this.selected = Math.floor(Math.random() * this.dictionary.length)
      this.input = null
    }
  },
  created() {
    this.dictionary = dictionary
      .split('\n')
      .filter((x) => x !== '')
      .map((x) => x.split(':').map((y) => y.trim()))

    this.selectNextEntry()
  }
}
</script>
