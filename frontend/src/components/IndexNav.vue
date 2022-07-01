<script setup>
import axios from 'axios';
import IndexNavLink from './IndexNavLink.vue';
</script>

<script>
export default {
  inject: ['login_state'],
  methods: {
    logout() {
      axios
        .post("/api/v1/logout")
        .then(_ => {
          this.login_state.is_logged_in = false;
          this.login_state.profile = null;
          this.$router.push("/");
        })
    }
  }
}
</script>

<template>
  <ul class="flex border-b px-5 pt-5">
    <IndexNavLink label="Home" link="/" />
    <IndexNavLink label="Sauces" link="/sauces" />
    <IndexNavLink v-if="login_state.is_logged_in" label="Submit review" link="/review/submit" />

    <div class="ml-auto flex">
      <IndexNavLink label="About" link="/about" />
      <IndexNavLink v-if="login_state.is_logged_in" label="Profile" link="/profile" />
      <IndexNavLink v-if="login_state.is_logged_in" label="Logout" @btn-click="logout" />
      <IndexNavLink v-else label="Login" link="/login" />
    </div>
  </ul>
</template>
