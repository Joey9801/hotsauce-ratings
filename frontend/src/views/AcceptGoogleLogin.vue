<script setup>
import axios from 'axios';
import LoadingSpinner from '../components/LoadingSpinner.vue';
</script>

<script>
export default {
  inject: ['login_state'],
  mounted() {
    let hash = window.location.hash;
    if (hash.length <= 0) {
      return
    }
    let params = new URLSearchParams(hash.slice(1))
    let google_id_token = params.get("id_token")
    
    axios
      .post("/api/v1/login", { google_id_token })
      .then(_ => {
        this.login_state.is_logged_in = true;
        axios
          .get("/api/v1/basic_profile")
          .then(response => {
            this.login_state.profile = response.data
            this.$router.push("/profile")
          })
      }, error => {
        if (error.response) {
          if (error.response.data.error == "No account exists for the given credentials") {
            this.$router.push("/signup?google_id_token=" + google_id_token)
          }
        }
      })
  }
}
</script>

<template>
<LoadingSpinner />
</template>