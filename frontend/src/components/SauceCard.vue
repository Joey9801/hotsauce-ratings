<script>
export default {
    props: ['sauce_name', 'manufacturer_name']
}
</script>

<template>
    <div class="flex flex-col w-full text-left rounded-xl border shadow-md hover:shadow-xl transition duration-100 cursor-pointer hover:scale-105">
        <img class="rounded-t-xl" src="../assets/aka_miso.jpg" />
        <div class="px-5 pb-5 flex flex-col flex-grow">
            <div class="text-lg">
                {{ sauce_name }}
            </div>
            <div class="text-slate-700 text-md mt-auto">
                {{ manufacturer_name }}
            </div>
        </div>
    </div>
</template>