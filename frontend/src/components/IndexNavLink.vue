<script>
export default {
  props: ['label', 'link']
}
</script>

<template>
  <li class="mr-1">
    <div v-if="link != null">
      <router-link :to="link" class="bg-white inline-block py-2 px-4 mx-2 text-slate-700 font-semibold select-none hover:bg-slate-100 rounded-t-xl transition duration-100">
        {{ label }}
      </router-link>
    </div>
    <div v-else>
      <p class="bg-white inline-block py-2 px-4 mx-2 text-slate-700 font-semibold select-none hover:bg-slate-100 rounded-t-xl transition duration-100 cursor-pointer" @click="$emit('btn-click')">
        {{ label }}
      </p>
    </div>
  </li>
</template>

<style>
.router-link-exact-active {
  @apply bg-slate-200 hover:bg-slate-200;
}
</style>