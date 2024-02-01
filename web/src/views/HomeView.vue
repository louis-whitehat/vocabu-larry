<template>
  <main style="font-size: larger">
    <div><label>Translate:</label> {{ word }}</div>
    
    <form @submit.prevent="submit">
      <input type="text" v-model="input" />
      <button style="margin-left: 20px" @click="submit">Submit</button>
    </form>

    <div style="margin-top: 50px">
      <img src="../assets/checkmark.png" v-if="answerCorrect === true" />
      <img src="../assets/red-cross.png" v-if="answerCorrect === false" />
    </div>
  </main>
</template>

<script>
export default {
  name: 'HomeView',
  data() {
    return {
      dictionary: [
        ['Haus', 'house'],
        ['schwarz', 'black'],
        ['schnell', 'fast']
      ],
      input: null,
      answerCorrect: null
    }
  },
  computed: {
    word() {
      return this.dictionary[0][0]
    }
  },
  methods: {
    submit() {
      this.answerCorrect = this.dictionary[0][1] == this.input
    }
  }
}
</script>
