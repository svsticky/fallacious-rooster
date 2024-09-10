import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'

const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    component: () => import('@/layouts/UnauthorizedMiddleware.vue'),
    children: [
      {
        path: '',
        component: () => import('@/views/HomeView.vue')
      }
    ]
  },
  {
    path: '/admin',
    component: () => import('@/layouts/AuthorizedAdminMiddleware.vue'),
    children: [
      {
        path: 'settings',
        component: () => import('@/views/SettingsView.vue'),
      }
    ]
  }
]

const router = createRouter({
  history: createWebHistory('/'),
  routes,
})

export default router
