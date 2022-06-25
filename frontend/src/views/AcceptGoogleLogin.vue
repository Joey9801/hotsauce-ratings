<script setup>
import axios from 'axios';
import LoadingSpinner from '../components/LoadingSpinner.vue';
</script>

<script>
export default {
  inject: ['loginState'],
  mounted() {
    let hash = window.location.hash;
    if (hash.length <= 0) {
      return
    }
    let params = new URLSearchParams(hash.slice(1))

    console.log(this.loginState)
    axios.post("/api/v1/login", { google_access_token: params.get("id_token") })
      .then(_ => {
        this.loginState.isLoggedIn = true;
        return axios.get("/api/v1/basic_profile");
      })
      .then(response => {
        this.loginState.profile = response.data
        this.$router.push("/profile")
      })
  }
}
</script>

<template>
<LoadingSpinner />
</template>