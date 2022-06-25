import * as Vue from 'vue'
import * as VueRouter from 'vue-router'
import './index.css'

import App from './App.vue'
import Home from './views/Home.vue'
import About from './views/About.vue'

import Login from '@/views/Login.vue'
import AcceptGoogleLogin from '@/views/AcceptGoogleLogin.vue'
import Signup from '@/views/Signup.vue'

import Profile from '@/views/Profile.vue'

import Sauces from '@/views/Sauces.vue'
import ReviewSubmit from './views/ReviewSubmit.vue'

import { reactive } from 'vue'

const routes = [
    { path: '/', component: Home },
    { path: '/about', component: About },
    { path: '/login', component: Login },
    { path: '/acceptGoogleLogin', component: AcceptGoogleLogin },
    { path: '/signup', component: Signup, props: route => ({ google_id_token: route.query.google_id_token })},
    { path: '/profile', component: Profile },
    { path: '/sauces', component: Sauces },
    { path: '/review/submit', component: ReviewSubmit },
  ]

const router = VueRouter.createRouter({
    history: VueRouter.createWebHistory(),
    routes,
})

const app = Vue.createApp(App)
app.use(router)

app.provide("login_state", reactive({ is_logged_in: false, profile: null }))

app.mount('#app')