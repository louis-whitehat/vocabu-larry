<template>
  <div>
    <h1>Welcome to Vocabu-Larry ;-)</h1>

    <br />

    <table style="margin: 0 auto">
      <tr>
        <td class="label">
          <label style="font-weight: bold">Who are you? </label>
        </td>
        <td class="value">
          <select v-model="name" class="browser-default">
            <option v-for="item in users" v-bind:key="item.name" :value="item.name">
              {{ item.name }}
            </option>
          </select>
        </td>
      </tr>

      <tr v-if="name">
        <td class="label">
          <label style="font-weight: bold">Choose a dictionary: </label>
        </td>
        <td class="value">
          <select v-model="dictionary" class="browser-default">
            <option v-for="item in dictionaries" v-bind:key="item" :value="item">
              {{ item }}
            </option>
          </select>
        </td>
      </tr>

      <tr v-if="name">
        <td class="label">Score</td>
        <td class="value">
          <router-link :to="{ name: 'score', params: { user: name } }">
            <button>Score</button>
          </router-link>
        </td>
      </tr>
    </table>
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

const dictionaries = computed(() => {
  const user = users.value.find((x) => x.name === name.value)
  return user ? user.dictionaries : []
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
.label {
  width: 200px;
  text-align: left;
}
.value {
  width: 200px;
  text-align: left;
}
</style>
