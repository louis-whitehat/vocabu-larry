import { createRouter, createWebHistory } from 'vue-router'
import LoginView from './views/LoginView.vue'
import ExamView from './views/ExamView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'login',
      component: LoginView
    },
    {
      path: '/exam/:dictionaryName',
      name: 'exam',
      component: ExamView,
      props: true
    }
  ]
})

export default router
