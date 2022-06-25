<script setup>
import axios from 'axios';
import LoadingSpinner from '../components/LoadingSpinner.vue'
</script>

<script>
export default {
  inject: ["login_state"],
  props: ["google_id_token"],
  data() {
    return {
      username: "",
      signup_button_disabled: false,
      username_error: null,
    };
  },
  mounted() {
    if (this.login_state.is_logged_in) {
      this.$router.push("/");
    }
  },
  methods: {
    doSignUp() {
      console.log("Doing signup with username = ", this.username);
      this.signup_button_disabled = true;
      axios
        .post("/api/v1/signup", { google_id_token: this.google_id_token, username: this.username })
        .then(_ => {
          this.login_state.is_logged_in = true;
          axios
            .get("/api/v1/basic_profile")
            .then(response => {
              this.login_state.profile = response.data
              this.$router.push("/profile")
            })
        }, error => {
          console.log(error)
          if (error.response) {
            this.username_error = error.response.data.details
            console.log(this.username_error)
          }
          this.signup_button_disabled = false;
        })
    }
  }
}
</script>

<template>
  <button @click="signup_button_disabled = !signup_button_disabled">toggle disable</button>

  <div class="mx-auto block p-6 rounded-xl shadow-md bg-white max-w-sm border">
    <p class="pb-5">
      This is your first time logging in, please choose a username to finish signing up.
    </p>
    <div>
      <input type="text"
        class="form-control block w-full px-3 py-1.5 font-normal text-gray-700 bg-white bg-clip-padding border border-solid border-gray-300 rounded transition ease-in-out m-0 focus:text-gray-700 focus:bg-white focus:border-blue-600 focus:outline-none"
        id="username" placeholder="Enter username" v-model="username">
        
      <p class="text-sm text-red-600"> {{ username_error }}</p>

      <div class="flex">
        <button class="mt-5 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
          :class="signup_button_disabled ? 'bg-blue-200 hover:bg-blue-200' : ''" @click="doSignUp"
          :disabled="signup_button_disabled">
          Sign up
        </button>

        <div class="w-fit mt-5 ml-5">
          <LoadingSpinner v-if="signup_button_disabled" />
        </div>
      </div>

    </div>
  </div>
</template>