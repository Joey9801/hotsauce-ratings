<script setup>
import IndexNavButton from './IndexNavButton.vue';
</script>

<script>
export default {
  inject: ['login_state']
}
</script>

<template>
  <ul class="flex border-b px-5 pt-5">
    <IndexNavButton label="Home" link="/" />
    <IndexNavButton label="Sauces" link="/sauces" />
    <IndexNavButton v-if="login_state.is_logged_in" label="Submit review" link="/review/submit" />
    <div class="ml-auto flex">
    <IndexNavButton label="About" link="/about" />

    <IndexNavButton v-if="login_state.is_logged_in" label="Profile" link="/profile" />
    <IndexNavButton v-else label="Login" link="/login" />
    </div>
  </ul>
</template>
