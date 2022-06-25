<script setup>
import axios from "axios";
import IndexNav from "./components/IndexNav.vue";
</script>

<script>
export default {
  inject: ["login_state"],
  mounted() {
    axios.get("/api/v1/basic_profile")
      .then(response => {
        this.login_state.is_logged_in = true;
        this.login_state.profile = response.data
      })
  }
}
</script>


<template>
  <div class="h-max m-5 max-w-6xl mx-auto rounded-xl shadow-xl bg-white">
    <IndexNav />
    <div class="w-full p-10">
      <router-view></router-view>
    </div>
  </div>
</template>