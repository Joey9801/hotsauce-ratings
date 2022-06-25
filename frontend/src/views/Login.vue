<script setup>
import axios from 'axios';
import { inject } from 'vue'
const loginState = inject('loginState');
</script>

<script>
export default {
    data() {
        return {
            google_oidc_config: {},
            google_client_id: "1029137063431-crnebmaeal8jdm85iurqoin9k6aqvccj.apps.googleusercontent.com",
            logged_in_name: null,
        }
    },
    mounted() {
        axios
            .get("https://accounts.google.com/.well-known/openid-configuration")
            .then(response => {
                this.google_oidc_config = response.data;
            })
            
        axios
            .get("/api/v1/my_name")
            .then(response => this.logged_in_name = response.data)
    },
    methods: {
        doGoogleLogin() {
            var urlParams = {
                client_id: this.google_client_id,
                redirect_uri: new URL(window.location.href).origin + "/acceptGoogleLogin",
                response_type: "token id_token",
                scope: "openid profile email",
                state: "google",
                nonce: "asdf",
            }
            
            var url = new URL(this.google_oidc_config.authorization_endpoint);
            url.search = new URLSearchParams(urlParams);

            window.location.href = url;
        }
    }
}
</script>

<template>
    <h1 class="text-5xl font-bold pb-10">Login</h1>

    <h1 class="text-3xl font-bold p-10">Logged in name:</h1>
    <p>{{ logged_in_name ?? "not logged in" }}</p>

    <h1 class="text-3xl font-bold p-10">Google OIDC config</h1>
    <table class="table-auto">
        <tbody class="bg-white dark:bg-slate-800">
            <tr>
                <td>Issuer</td>
                <td>{{ google_oidc_config.issuer }}</td>
            </tr>
            <tr>
                <td>Authorization Endpoint</td>
                <td>{{ google_oidc_config.authorization_endpoint }}</td>
            </tr>
            <tr>
                <td>Token Endpoint</td>
                <td>{{ google_oidc_config.token_endpoint }}</td>
            </tr>
            <tr>
                <td>Jwks URI</td>
                <td>{{ google_oidc_config.jwks_uri }}</td>
            </tr>
        </tbody>
    </table>

    <h1 class="text-3xl font-bold p-10">Login state</h1>
    <table class="table-auto">
        <tbody class="bg-white dark:bg-slate-800">
            <tr>
                <td>isLoggedIn</td>
                <td>{{ loginState.isLoggedIn }}</td>
            </tr>
            <tr>
                <td>provider</td>
                <td>{{ loginState.provider ?? "null" }}</td>
            </tr>
            <tr>
                <td>access_token</td>
                <td>{{ loginState.access_token ?? "null" }}</td>
            </tr>
            <tr>
                <td>id_token</td>
                <td>{{ loginState.id_token ?? "null" }}</td>
            </tr>
        </tbody>
    </table>


    <button class="mt-5 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
        @click="doGoogleLogin">Do google login</button>
</template>

<style>
td {
    @apply border-b border-slate-100 p-4 pl-8 pr-8 text-slate-500 max-w-2xl;
    overflow-wrap: anywhere;
}
</style>