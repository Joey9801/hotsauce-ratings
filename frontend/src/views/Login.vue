<script setup>
import axios from 'axios';
import { inject } from 'vue'
</script>

<script>
export default {
    inject: ["login_state"],
    data() {
        return {
            google_oidc_config: {
                client_id: "1029137063431-crnebmaeal8jdm85iurqoin9k6aqvccj.apps.googleusercontent.com",
                auth_endpoint: "https://accounts.google.com/o/oauth2/v2/auth",
            },
        }
    },
    methods: {
        do_google_login() {
            var urlParams = {
                client_id: this.google_oidc_config.client_id,
                redirect_uri: new URL(window.location.href).origin + "/acceptGoogleLogin",
                response_type: "token id_token",
                scope: "openid profile email",
                state: "google",
                nonce: "asdf",
            }

            var url = new URL(this.google_oidc_config.auth_endpoint);
            url.search = new URLSearchParams(urlParams);

            window.location.href = url;
        }
    }
}
</script>

<template>
    <div class="flex">
        <button class="mx-auto mt-5 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
            @click="do_google_login">Login with google</button>
    </div>
</template>

<style>
td {
    @apply border-b border-slate-100 p-4 pl-8 pr-8 text-slate-500 max-w-2xl;
    overflow-wrap: anywhere;
}
</style>