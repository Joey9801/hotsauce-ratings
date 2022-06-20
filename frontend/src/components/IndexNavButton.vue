<script>
export default {
    props: ['label', 'link']
}
</script>

<template>
  <li class="mr-1">
    <router-link
        :to="link"
        class="bg-white inline-block py-2 px-4 mx-2 text-slate-700 font-semibold" >
            {{ label }}
    </router-link>
  </li>
</template>

<style>
.router-link-exact-active {
    @apply text-slate-600 bg-slate-100 rounded-t-xl;
}
</style>