<script setup>
import { inject } from 'vue'
const login_state = inject('login_state');
</script>

<template>
    <h1 class="text-5xl font-bold pb-10">Profile</h1>
    <table class="table-auto">
        <tbody class="bg-white dark:bg-slate-800">
            <tr v-for="(value, prop) in login_state.profile" :key="prop">
                <td>{{ prop }}</td>
                <td>{{ value }}</td>
            </tr>
        </tbody>
    </table>
</template>

<style>
td {
    @apply border-b border-slate-100 p-4 pl-8 pr-8 text-slate-500 max-w-2xl;
    overflow-wrap: anywhere;
}
</style>