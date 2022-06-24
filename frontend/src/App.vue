<script setup>
import { defineComponent, provide, ref } from "vue";
import IndexNav from "./components/IndexNav.vue";


const loginState = ref({
  isLoggedIn: false,
});
provide("loginState", loginState);
</script>


<template>
  <div class="h-max m-5 max-w-6xl mx-auto rounded-xl shadow-xl bg-white">
    <IndexNav />
    <div class="w-full p-10">
      <router-view></router-view>
    </div>
  </div>
</template>