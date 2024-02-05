<template>
  <main :class="status" style="font-size: larger;padding:20px">
    <div><label style="font-weight: bold;">Translate this:</label> {{ word }}</div>

    <form @submit.prevent="submit">
      <input type="text" v-model="input" />
      <button style="margin-left: 20px">Submit</button>
    </form>

    <div style="margin-top: 50px">
      <div v-if="answerCorrect === false">
        <span style="padding-left: 50px">Correct answer was: {{ previousCorrect }}</span>
      </div>
    </div>
  </main>
</template>

<script>
import dictionary from '../../../dictionaries/french.txt?raw'

export default {
  name: 'HomeView',
  data() {
    return {
      dictionary: null,
      word: null,
      translation: null,
      previousCorrect: null,
      input: null,
      answerCorrect: null
    }
  },
  computed: {
    status() { return this.answerCorrect ? 'correct' : 'wrong' }
  },
  methods: {
    submit() {
      this.previousCorrect = this.translation
      this.answerCorrect = this.translation == this.input
      this.selectNextEntry()
    },
    selectNextEntry() {
      const selected = Math.floor(Math.random() * this.dictionary.length)
      this.word = this.dictionary[selected][0]
      this.translation = this.dictionary[selected][1]
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

<style scoped>
.correct {
  background-color: green;
}
.wrong {
  background-color: red;
}
</style>