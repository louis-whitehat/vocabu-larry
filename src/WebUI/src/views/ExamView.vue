<template>
  <div class="page-shell exam-page">
    <section class="panel-card exam-card">
      <h1 class="page-title">Exam</h1>
      <p class="page-copy">Answer the translation and keep the streak moving.</p>

      <div class="exam-question">
        <span class="muted-note">What is the translation of</span>
        <span class="word">{{ word }}</span>
        <span class="hint-pill">Hint: {{ numWords }} word(s)</span>
      </div>

      <form @submit.prevent="submit" class="exam-form">
        <input type="text" v-model="input" class="answer-input" />
        <button type="submit">Submit</button>
        <span class="score-chip">{{ correctCount }} / {{ totalCount }}</span>
      </form>

      <div class="feedback-panel" :class="status">
        <div v-if="answerCorrect === true">Correct 👍😉</div>
        <div v-if="answerCorrect === false">
          Sorry 🙁 Your answer <span class="word">{{ yourAnswer }}</span> is not correct, correct
          answer would have been <span class="word">{{ previousCorrect }}</span>
        </div>
      </div>

      <div class="actions-row">
        <router-link :to="{ name: 'score', params: { user: route.params.user } }" class="secondary-action">
          Finished
        </router-link>
      </div>
    </section>
  </div>
</template>

<script setup>
  import { ref, computed, onMounted } from 'vue'
  import { useRoute } from 'vue-router'
  import api from '@/api.js'

  const route = useRoute()

  const dictionary = ref([])
  const word = ref(null)
  const numWords = ref(null)
  const translation = ref(null)
  const yourAnswer = ref(null)
  const previousCorrect = ref(null)
  const input = ref(null)
  const answerCorrect = ref(null)
  const totalCount = ref(0)
  const correctCount = ref(0)
  const failureCounts = ref(new Map())

  const getEntryKey = (entry) => {
    return `${entry.word}\n${entry.translation}`
  }

  const getEntryWeight = (entry) => {
    const failures = failureCounts.value.get(getEntryKey(entry)) ?? 0
    return 1 + failures
  }

  const selectWeightedEntry = (entries) => {
    const totalWeight = entries.reduce((sum, entry) => sum + getEntryWeight(entry), 0)
    let remainingWeight = Math.random() * totalWeight

    for (const entry of entries) {
      remainingWeight -= getEntryWeight(entry)

      if (remainingWeight < 0) {
        return entry
      }
    }

    return entries[entries.length - 1]
  }

  const normalizeAnswer = (value) => {
    return String(value ?? '')
      .trim()
      .split(/\s+/)
      .filter(Boolean)
      .join(' ')
      .toLowerCase()
  }

  const stripLeadingToken = (value, token) => {
    return value.startsWith(`${token} `) ? value.slice(token.length + 1) : value
  }

  const getDictionaryLanguage = (dictionaryName) => {
    const [language] = String(dictionaryName ?? '').split('.', 1)
    const normalizedLanguage = language.trim().toLowerCase()

    if (normalizedLanguage === 'english' || normalizedLanguage === 'french') {
      return normalizedLanguage
    }

    return null
  }

  const answersMatch = (expectedAnswer, actualAnswer, dictionaryName) => {
    const normalizedExpected = normalizeAnswer(expectedAnswer)
    const normalizedActual = normalizeAnswer(actualAnswer)

    if (normalizedExpected === normalizedActual) {
      return true
    }

    if (getDictionaryLanguage(dictionaryName) !== 'english') {
      return false
    }

    return (
      stripLeadingToken(normalizedExpected, 'to') === stripLeadingToken(normalizedActual, 'to') ||
      stripLeadingToken(normalizedExpected, 'the') === stripLeadingToken(normalizedActual, 'the')
    )
  }

  const status = computed(() => {
    return answerCorrect.value === true ? 'correct' : answerCorrect.value === false ? 'wrong' : ''
  })

  const submit = async () => {
    previousCorrect.value = translation.value
    yourAnswer.value = input.value
    answerCorrect.value = answersMatch(translation.value, input.value, route.params.dictionary)

    if (!answerCorrect.value) {
      const entryKey = getEntryKey({ word: word.value, translation: translation.value })
      const failures = failureCounts.value.get(entryKey) ?? 0
      failureCounts.value.set(entryKey, failures + 1)
    }

    totalCount.value += 1
    if (answerCorrect.value) {
      correctCount.value += 1
    }

    await api.post('/api/score', {
      user: route.params.user,
      dictionary: route.params.dictionary,
      isCorrect: answerCorrect.value
    })

    selectNextEntry()
  }

  const selectNextEntry = () => {
    if (dictionary.value.length === 0) {
      word.value = null
      translation.value = null
      numWords.value = null
      input.value = null
      return
    }

    const selectedEntry = selectWeightedEntry(dictionary.value)
    word.value = selectedEntry.word
    translation.value = selectedEntry.translation

    numWords.value = translation.value.trim().split(/\s+/).length

    input.value = null
  }

  onMounted(async () => {
    answerCorrect.value = null
    correctCount.value = 0
    totalCount.value = 0
    failureCounts.value = new Map()

    try {
      const response = await api.get('/api/dictionary', {
        params: {
          user: route.params.user,
          dictionary: route.params.dictionary
        }
      })

      dictionary.value = response.data

      selectNextEntry()
    } catch (error) {
      console.error('Error fetching dictionary:', error)
    }
  })
</script>

<style scoped>
  .exam-card {
    font-size: 1.1rem;
  }

  .exam-question {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 14px;
    margin-bottom: 18px;
  }

  .hint-pill {
    display: inline-flex;
    align-items: center;
    padding: 6px 12px;
    border-radius: 999px;
    background: rgba(92, 148, 220, 0.12);
    color: #2d5f99;
    font-size: 0.92rem;
    font-weight: 700;
  }

  .exam-form {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 14px;
  }

  .answer-input {
    flex: 1 1 280px;
    margin: 0;
  }

  .score-chip {
    display: inline-flex;
    align-items: center;
    padding: 6px 12px;
    border-radius: 999px;
    background: rgba(36, 93, 147, 0.1);
    color: #244970;
    font-weight: 700;
  }

  .feedback-panel {
    margin-top: 22px;
    padding: 20px;
    border-radius: 18px;
    background: rgba(255, 255, 255, 0.6);
  }

  .correct {
    background-color: rgba(54, 176, 90, 0.18);
    color: #17592b;
  }

  .wrong {
    background-color: rgba(221, 77, 77, 0.16);
    color: #8d1f1f;
  }

  .word {
    font-weight: bold;
    color: #173a63;
  }

  @media (max-width: 640px) {
    .exam-form {
      align-items: stretch;
    }
  }
</style>
