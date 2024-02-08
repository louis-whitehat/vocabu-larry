<template>
  <main :class="status" style="font-size: larger; padding: 20px">
    <div>
      <label>What is the translation of </label>'<span style="font-weight: bold">{{ word }}</span
      >'?
    </div>

    <form @submit.prevent="submit" style="margin-top: 10px">
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
export default {
  name: 'HomeView',
  props: ['dictionaryFile'],
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
    status() {
      return this.answerCorrect === true ? 'correct' : this.answerCorrect === false ? 'wrong' : ''
    }
  },
  watch: {
    dictionaryFile() {
      this.answerCorrect = null
      this.dictionaryChanged()
    }
  },
  methods: {
    submit() {
      this.previousCorrect = this.translation
      this.answerCorrect = this.translation.toLowerCase() == this.input.toLowerCase()
      this.selectNextEntry()
    },
    selectNextEntry() {
      const selected = Math.floor(Math.random() * this.dictionary.length)
      this.word = this.dictionary[selected][0]
      this.translation = this.dictionary[selected][1]
      this.input = null
    },
    dictionaryChanged() {
      if (this.dictionaryFile) {
        this.dictionary = this.dictionaryFile.content
          .split('\n')
          .filter((x) => x !== '')
          .map((x) => x.split(':').map((y) => y.trim()))

        this.selectNextEntry()
      }
    }
  },
  created() {
    this.dictionaryChanged()
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
