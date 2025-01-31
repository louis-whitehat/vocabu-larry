<template>
  <div style="font-size: larger; padding: 20px">
    <div>
      What is the translation of:
      <span class="word">{{ word }}</span>
    </div>

    <form @submit.prevent="submit" style="margin-top: 10px">
      <input type="text" v-model="input" />
      <button style="margin-left: 20px">Submit</button>
      <span style="margin-left: 50px">{{ correctCount }} / {{ totalCount }}</span>
    </form>

    <div style="margin-top: 20px; padding: 20px" :class="status">
      <div v-if="answerCorrect === true">Correct 👍😉</div>
      <div v-if="answerCorrect === false">
        Sorry 🙁 Your answer <span class="word">{{ yourAnswer }}</span> is not correct, correct
        answer would have been <span class="word">{{ previousCorrect }}</span>
      </div>
    </div>

    <div style="margin-top: 20px">
      <router-link :to="{ name: 'score', params: { user: this.$route.params.user } }">
        <button>Finished</button>
      </router-link>
    </div>
  </div>
</template>

<script setup>
  import { ref, computed, onMounted } from 'vue'
  import { useRoute } from 'vue-router'
  import api from '@/api.js'

  const route = useRoute()

  const dictionary = ref([])
  const word = ref(null)
  const translation = ref(null)
  const yourAnswer = ref(null)
  const previousCorrect = ref(null)
  const input = ref(null)
  const answerCorrect = ref(null)
  const totalCount = ref(0)
  const correctCount = ref(0)

  const status = computed(() => {
    return answerCorrect.value === true ? 'correct' : answerCorrect.value === false ? 'wrong' : ''
  })

  const submit = async () => {
    previousCorrect.value = translation.value
    yourAnswer.value = input.value
    answerCorrect.value = translation.value.toLowerCase() === input.value.toLowerCase()

    totalCount.value += 1
    if (answerCorrect.value) {
      correctCount.value += 1
    }

    await api.post('/api/score', {
      user: route.params.user,
      isCorrect: answerCorrect.value
    })

    selectNextEntry()
  }

  const selectNextEntry = () => {
    const selected = Math.floor(Math.random() * dictionary.value.length)
    word.value = dictionary.value[selected][0]
    translation.value = dictionary.value[selected][1]
    input.value = null
  }

  onMounted(async () => {
    answerCorrect.value = null
    correctCount.value = 0
    totalCount.value = 0

    try {
      const response = await api.get('/api/dictionary', {
        params: {
          user: route.params.user,
          dictionary: route.params.dictionary
        }
      })

      dictionary.value = response.data
        .split('\n')
        .filter((x) => x !== '')
        .map((x) => x.split(':').map((y) => y.trim()))

      selectNextEntry()
    } catch (error) {
      console.error('Error fetching dictionary:', error)
    }
  })
</script>

<style scoped>
  .correct {
    background-color: rgb(19, 201, 19);
  }
  .wrong {
    background-color: rgb(234, 18, 18);
  }
  .word {
    font-weight: bold;
  }
</style>
