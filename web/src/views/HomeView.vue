<template>
  <main style="font-size: larger">
    <div><label style="font-weight: bold;">Translate this:</label> {{ word }}</div>

    <form @submit.prevent="submit">
      <input type="text" v-model="input" />
      <button style="margin-left: 20px">Submit</button>
    </form>

    <div style="margin-top: 50px">
      <img src="../assets/checkmark.png" v-if="answerCorrect === true" />
      <div v-if="answerCorrect === false">
        <img src="../assets/red-cross.png" />
        <span style="padding-left: 50px">{{ translation }}</span>
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
      selected: null,
      input: null,
      answerCorrect: null
    }
  },
  computed: {
    word() {
      return this.dictionary[this.selected][0]
    },
    translation() {
      return this.dictionary[this.selected][1]
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
