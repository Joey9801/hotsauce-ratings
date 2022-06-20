<script setup>
import SauceCard from '@/components/SauceCard.vue'
import LoadingSpinner from '@/components/LoadingSpinner.vue'
import axios from 'axios'
</script>

<script>
export default {
    data() {
        return {
            loadingSauces: true,
            loadingManufacturers: true,
            sauces: [],
            manufacturers: {}
        }
    },
    computed: {
        loading() {
            return this.loadingSauces || this.loadingManufacturers
        }
    },
    mounted() {
        axios
            .get("http://localhost:3030/api/sauce")
            .then(response => {
                this.sauces = response.data;
                this.loadingSauces = false;
            })

        axios
            .get("http://localhost:3030/api/manufacturer")
            .then(response => {
                response.data.forEach(m => {
                    this.manufacturers[m.manufacturer_id] = m.manufacturer_name
                });
                this.loadingManufacturers = false;
            })
    }
}
</script>

<template>
    <h1 class="text-5xl font-bold pb-10">All Sauces</h1>
    <LoadingSpinner v-if="loading" />
    <div class="grid grid-cols-4 gap-10 justify-items-center">
        <SauceCard v-for="sauce in sauces" :sauce_name="sauce.sauce_name"
            :manufacturer_name="manufacturers[sauce.manufacturer_id] || '<unknown>'" />
        <SauceCard v-for="sauce in sauces" :sauce_name="sauce.sauce_name"
            :manufacturer_name="manufacturers[sauce.manufacturer_id]" />
        <SauceCard v-for="sauce in sauces" :sauce_name="sauce.sauce_name"
            :manufacturer_name="manufacturers[sauce.manufacturer_id]" />
        <SauceCard v-for="sauce in sauces" :sauce_name="sauce.sauce_name"
            :manufacturer_name="manufacturers[sauce.manufacturer_id]" />
    </div>
</template>