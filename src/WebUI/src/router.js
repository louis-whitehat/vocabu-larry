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

export default router
