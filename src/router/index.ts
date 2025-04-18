import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView,
    },
    {
      path: '/projects',
      name: 'projects',
      component: () => import('../views/ProjectsView.vue'),
    },
    {
      path: '/gallery',
      name: 'gallery',
      component: () => import('../views/GalleryView.vue'),
    },
    {
      path: '/blog',
      name: 'blog',
      component: () => import('../views/BlogView.vue'),
    },
    {
      path: '/blog/hkcert.md',
      name: 'blog/hkcert.md',
      component: () => import('../blogs/hkcert.vue'),
    },
    { 
      path: "/:pathMatch(.*)*",
      name: "ERROR_LOST_404",
      component: () => import('../views/404View.vue'), 
    },
    
  ],
})

export default router
