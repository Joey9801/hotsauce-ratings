<script setup>
import IndexNavButton from './IndexNavButton.vue';
</script>

<template>
  <ul class="flex border-b px-5 pt-5">
    <IndexNavButton label="Home" link="/" />
    <IndexNavButton label="Sauces" link="/sauces" />
    <IndexNavButton label="Submit review" link="/review/submit" />
    <div class="ml-auto">
    <IndexNavButton label="About" link="/about" />
    </div>
  </ul>
</template>
