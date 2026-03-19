<template>
  <div class="login-page">
    <header class="toolbar">
      <div class="brand">
        <span class="brand-kicker">Vocabulary trainer</span>
        <h1>Vocabu-Larry</h1>
      </div>

      <router-link :to="{ name: 'logs' }" class="toolbar-action" aria-label="Open logs" title="Logs">
        <svg viewBox="0 0 24 24" aria-hidden="true">
          <path
            d="M6 3h9l5 5v13a1 1 0 0 1-1 1H6a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2Zm8 1.5V9h4.5M8 13h8M8 17h8M8 9h3"
          />
        </svg>
      </router-link>
    </header>

    <section class="hero-card">
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
  .login-page {
    max-width: 860px;
    margin: 0 auto;
    padding: 12px 12px 40px;
  }

  .toolbar {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: 20px;
    margin-bottom: 32px;
  }

  .brand-kicker {
    display: inline-block;
    margin-bottom: 6px;
    font-size: 0.8rem;
    font-weight: 700;
    letter-spacing: 0.16em;
    text-transform: uppercase;
    color: #4a79b6;
  }

  .brand h1 {
    margin: 0;
    color: #173a63;
  }

  .toolbar-action {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 48px;
    height: 48px;
    border: 1px solid rgba(54, 105, 170, 0.2);
    border-radius: 999px;
    background: rgba(242, 248, 255, 0.96);
    color: #2d5f99;
    box-shadow: 0 10px 24px rgba(27, 73, 125, 0.1);
    transition:
      transform 140ms ease,
      box-shadow 140ms ease,
      border-color 140ms ease;
  }

  .toolbar-action:hover {
    transform: translateY(-1px);
    border-color: rgba(54, 105, 170, 0.38);
    box-shadow: 0 14px 30px rgba(27, 73, 125, 0.16);
  }

  .toolbar-action svg {
    width: 20px;
    height: 20px;
    fill: none;
    stroke: currentColor;
    stroke-linecap: round;
    stroke-linejoin: round;
    stroke-width: 1.8;
  }

  .hero-card {
    padding: 28px;
    border: 1px solid rgba(54, 105, 170, 0.14);
    border-radius: 28px;
    background:
      radial-gradient(circle at top right, rgba(163, 210, 255, 0.52), transparent 28%),
      linear-gradient(180deg, rgba(247, 251, 255, 0.98), rgba(238, 246, 255, 0.98));
    box-shadow: 0 24px 60px rgba(27, 73, 125, 0.12);
    text-align: left;
  }

  .hero-copy {
    margin: 0 0 24px;
    max-width: 40rem;
    color: #395b84;
    font-size: 1.02rem;
  }

  .form-grid {
    display: grid;
    grid-template-columns: minmax(150px, 220px) minmax(0, 1fr);
    align-items: center;
    gap: 14px 20px;
  }

  .field-label {
    font-weight: 700;
    color: #244970;
    text-align: left;
  }

  .actions-row {
    margin-top: 24px;
    display: flex;
    justify-content: flex-start;
  }

  .secondary-action {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 10px 16px;
    border-radius: 999px;
    background: #245d93;
    color: #f4f9ff;
    text-decoration: none;
    font-weight: 700;
  }

  .secondary-action:hover {
    background: #1d4f7d;
    color: #f4f9ff;
  }

  @media (max-width: 640px) {
    .login-page {
      padding: 8px 0 28px;
    }

    .toolbar {
      margin-bottom: 20px;
    }

    .hero-card {
      padding: 22px 18px;
      border-radius: 22px;
    }

    .form-grid {
      grid-template-columns: 1fr;
      gap: 8px;
    }

    .actions-row {
      margin-top: 20px;
    }
  }
</style>
