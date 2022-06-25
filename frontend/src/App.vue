<script setup>
import IndexNav from "./components/IndexNav.vue";
</script>


<template>
  <div class="h-max m-5 max-w-6xl mx-auto rounded-xl shadow-xl bg-white">
    <IndexNav />
    <div class="w-full p-10">
      <router-view></router-view>
    </div>
  </div>
</template>