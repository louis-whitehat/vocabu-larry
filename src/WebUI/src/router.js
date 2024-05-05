import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'login',
      component: () => import('./views/LoginView.vue')
    },
    {
      path: '/exam/:name/:dictionaryName',
      name: 'exam',
      component: () => import('./views/ExamView.vue')
    }
  ]
})

export default router
