import Atable from '@/views/Atable.vue'
import Etable from '@/views/Etable.vue'
import Home from '@/views/Home.vue'
import Login from '@/views/Login.vue'
import Tables from '@/views/Tables.vue'
import Wtable from '@/views/Wtable.vue'
import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    { path: "/", component: Home },
    { path: "/login", component: Login },
    { path: "/tables/:target", component: Tables },
    { path: "/etable/:name", component: Etable },
    { path: "/wtable/:name", component: Wtable, },
    { path: "/atable/:name", component: Atable }
  ],
})

export default router
