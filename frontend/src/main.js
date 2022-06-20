import * as Vue from 'vue'
import * as VueRouter from 'vue-router'
import './index.css'

import App from './App.vue'
import Home from './views/Home.vue'
import About from './views/About.vue'
import Sauces from '@/views/Sauces.vue'
import ReviewSubmit from './views/ReviewSubmit.vue'

const routes = [
    { path: '/', component: Home },
    { path: '/about', component: About },
    { path: '/sauces', component: Sauces },
    { path: '/review/submit', component: ReviewSubmit },
  ]

const router = VueRouter.createRouter({
    history: VueRouter.createWebHashHistory(),
    routes,
})

const app = Vue.createApp(App)
app.use(router)
app.mount('#app')
