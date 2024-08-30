import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'

const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    component: () => import('@/layouts/Authorized.vue'),
    children: [
      {
        path: '',
        component: () => import('@/views/HomeView.vue')
      }
    ]
  },
]

const router = createRouter({
  history: createWebHistory('/'),
  routes,
})

export default router
