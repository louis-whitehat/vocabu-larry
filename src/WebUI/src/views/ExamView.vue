<template>
  <div>
    <div :class="status" style="font-size: larger; padding: 20px">
      <div>
        <label>What is the translation of </label><span class="word">{{ word }}</span
        >?
      </div>

      <form @submit.prevent="submit" style="margin-top: 10px">
        <input type="text" v-model="input" />
        <button style="margin-left: 20px">Submit</button>
        <span style="margin-left: 50px">{{ correctCount }} / {{ totalCount }}</span>
      </form>

      <div style="margin-top: 50px">
        <div v-if="answerCorrect === false">
          <span style="padding-left: 50px"
            >Correct answer would have been: <span class="word">{{ previousCorrect }}</span></span
          >
        </div>
      </div>
    </div>
    <div>
      <br />
      <router-link :to="{ name: 'score', params: { user: this.$route.params.user }}">
        <button>Finished</button>
      </router-link>
    </div>
  </div>
</template>

<script>
import api from '@/api.js'

export default {
  name: 'ExamView',
  data() {
    return {
      dictionary: null,
      word: null,
      translation: null,
      previousCorrect: null,
      input: null,
      answerCorrect: null,
      totalCount: 0,
      correctCount: 0
    }
  },
  computed: {
    status() {
      return this.answerCorrect === true ? 'correct' : this.answerCorrect === false ? 'wrong' : ''
    }
  },
  methods: {
    submit() {
      this.previousCorrect = this.translation
      this.answerCorrect = this.translation.toLowerCase() == this.input.toLowerCase()

      this.totalCount += 1
      if (this.answerCorrect) {
        this.correctCount += 1
      }

      api.post('/api/score', {
        user: this.$route.params.user,
        isCorrect: this.answerCorrect
      })

      this.selectNextEntry()
    },
    selectNextEntry() {
      const selected = Math.floor(Math.random() * this.dictionary.length)
      this.word = this.dictionary[selected][0]
      this.translation = this.dictionary[selected][1]
      this.input = null
    }
  },
  async created() {
    this.answerCorrect = null
    this.correctCount = 0
    this.totalCount = 0

    let response = await api.get('/api/dictionary', {
      params: {
        user: this.$route.params.user,
        dictionary: this.$route.params.dictionary
      }
    })

    this.dictionary = response.data
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
.word {
  font-weight: bold;
}
</style>
