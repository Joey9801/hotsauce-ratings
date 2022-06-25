<script setup>
import IndexNav from "./components/IndexNav.vue";

// import { provide, ref } from "vue";
// const loginState = ref({ isLoggedIn: false, profile: null });
// provide("loginState", loginState);
</script>


<template>
  <div class="h-max m-5 max-w-6xl mx-auto rounded-xl shadow-xl bg-white">
    <IndexNav />
    <div class="w-full p-10">
      <router-view></router-view>
    </div>
  </div>
</template>