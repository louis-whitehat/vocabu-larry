<template>
  <div class="login-page">
    <section class="panel-card">
      <h1 class="page-title">Welcome back</h1>
      <p class="hero-copy">Pick a learner, choose a dictionary, and jump straight into the next round.</p>

      <div class="form-grid">
        <label class="field-label" for="user-select">Who are you?</label>
        <select id="user-select" v-model="name">
          <option :value="null">Select a learner</option>
          <option v-for="item in users" :key="item.name" :value="item.name">
            {{ item.name }}
          </option>
        </select>

        <template v-if="name">
          <label class="field-label" for="dictionary-select">Choose a dictionary</label>
          <select id="dictionary-select" v-model="dictionary">
            <option :value="null">Select a dictionary</option>
            <option v-for="item in dictionaries" :key="item" :value="item">
              {{ item }}
            </option>
          </select>
        </template>
      </div>

      <div v-if="name" class="actions-row">
        <router-link :to="{ name: 'score', params: { user: name } }" class="secondary-action">
          Show score
        </router-link>
      </div>
    </section>
  </div>
</template>

<script setup>
  import { ref, computed, watch, onMounted } from 'vue'
  import { useRouter } from 'vue-router'
  import api from '@/api.js'

  const router = useRouter()

  const name = ref(null)
  const dictionary = ref(null)
  const users = ref([])
  const lastLoggedUser = ref(null)

  const dictionaries = computed(() => {
    const user = users.value.find((x) => x.name === name.value)
    return user ? user.dictionaries : []
  })

  watch(name, async (newName) => {
    dictionary.value = null

    if (!newName || newName === lastLoggedUser.value) {
      return
    }

    try {
      await api.post('/api/login', { user: newName })
      lastLoggedUser.value = newName
    } catch (error) {
      console.error('Error logging login event:', error)
    }
  })

  watch(dictionary, (newDictionary) => {
    if (newDictionary) {
      router.push({
        name: 'exam',
        params: { user: name.value, dictionary: newDictionary }
      })
    }
  })

  onMounted(async () => {
    try {
      const response = await api.get('/api/users')
      users.value = response.data
    } catch (error) {
      console.error('Error fetching users:', error)
    }
  })
</script>

<style scoped>
  .login-page {
    max-width: 860px;
    margin: 0 auto;
  }

  .hero-copy {
    margin: 0 0 24px;
  }
</style>
