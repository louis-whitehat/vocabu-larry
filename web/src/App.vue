<template>
  <div>
    <h1 style="text-align: center">Welcome to Vocabu-Larry ;-)</h1>
    <br />
    <label style="font-weight: bold">Dictionary: </label>
    <select v-model="dictionaryFile">
      <option v-for="item in dictionaryFiles" v-bind:key="item.name" :value="item">{{ item.name }}</option>
    </select>
    <br /><br />
    <home-view :dictionary-file="dictionaryFile" />
  </div>
</template>

<script>
import HomeView from './views/HomeView.vue'
import frenchDictionary from '../../dictionaries/french.txt?raw'
import englishDictionary from '../../dictionaries/english.txt?raw'

export default {
  name: 'AppView',
  components: { HomeView },
  data() {
    return {
      dictionaryFile: null,
      dictionaryFiles: [
        {
          name: 'french',
          content: frenchDictionary
        },
        {
          name: 'english',
          content: englishDictionary
        }
      ]
    }
  },
  created() {
    this.dictionaryFile = this.dictionaryFiles[0]
  }
}
</script>

<style scoped>
header {
  line-height: 1.5;
  max-height: 100vh;
}

nav {
  width: 100%;
  font-size: 12px;
  text-align: center;
  margin-top: 2rem;
}

nav a.router-link-exact-active {
  color: var(--color-text);
}

nav a.router-link-exact-active:hover {
  background-color: transparent;
}

nav a {
  display: inline-block;
  padding: 0 1rem;
  border-left: 1px solid var(--color-border);
}

nav a:first-of-type {
  border: 0;
}

@media (min-width: 1024px) {
  header {
    display: flex;
    place-items: center;
    padding-right: calc(var(--section-gap) / 2);
  }

  header .wrapper {
    display: flex;
    place-items: flex-start;
    flex-wrap: wrap;
  }

  nav {
    text-align: left;
    margin-left: -1rem;
    font-size: 1rem;

    padding: 1rem 0;
    margin-top: 1rem;
  }
}
</style>
