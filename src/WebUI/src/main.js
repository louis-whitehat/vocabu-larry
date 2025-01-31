import { createApp } from 'vue'
import { createRouter, createWebHistory } from 'vue-router'
import App from './App.vue'
import "@picocss/pico/css/pico.min.css";

const app = createApp(App)

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'login',
      component: () => import('./views/LoginView.vue')
    },
    {
      path: '/exam/:user/:dictionary',
      name: 'exam',
      component: () => import('./views/ExamView.vue')
    },
    {
      path: '/score/:user',
      name: 'score',
      component: () => import('./views/ScoreView.vue')
    }
  ]
})

app.use(router)

app.mount('#app')
