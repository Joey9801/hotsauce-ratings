<script setup>
import axios from 'axios';
import { inject } from 'vue'
import LoadingSpinner from '../components/LoadingSpinner.vue';
const loginState = inject('loginState');
</script>

<script>
export default {
  mounted() {
    let hash = window.location.hash;
    if (hash.length <= 0) {
      return
    }
    let params = new URLSearchParams(hash.slice(1))
    if (params.get("state") == "google") {
      this.loginState.provider = "google"
      this.loginState.access_token = params.get("access_token")
      this.loginState.id_token = params.get("id_token")
      this.loginState.isLoggedIn = true

      axios.post("/api/v1/login", { google_access_token: params.get("id_token") })
        .then(_ => {
          axios.get("/api/v1/my_name")
            .then(response => console.log(response))
        })

      this.$router.push("/login")
    }
    
  }
}
</script>

<template>
<LoadingSpinner />
</template>