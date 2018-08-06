<template>
<div>
    <Navbar/>
    <div class="h-screen flex justify-center mt-4 lg:mt-32">
        <div class="w-full max-w-xs">
            <form @submit.prevent="signup" class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4">
                <div class="mb-4">
                <label class="block text-grey-darker text-sm font-bold mb-2" for="username">
                    Username
                </label>
                <input id="username" name="username" type="text" placeholder="Username" v-model="form.username" class="shadow appearance-none border rounded w-full py-2 px-3 text-grey-darker leading-tight focus:outline-none focus:shadow-outline" >
                </div>
                <div class="mb-4">
                <label class="block text-grey-darker text-sm font-bold mb-2" for="username">
                    Email
                </label>
                <input id="email" type="email" name="email" placeholder="Email" v-model="form.email" class="shadow appearance-none border rounded w-full py-2 px-3 text-grey-darker leading-tight focus:outline-none focus:shadow-outline">
                </div>
                <div class="mb-6">
                <label class="block text-grey-darker text-sm font-bold mb-2" for="password">
                    Password
                </label>
                <input id="password" name="password" type="password" v-model="form.password" placeholder="******************" class="shadow appearance-none w-full py-2 px-3 text-grey-darker mb-3 leading-tight focus:outline-none focus:shadow-outline">
                <label class="block text-grey-darker text-sm font-bold mb-2" for="password">
                    Confirm your password
                </label>
                <input id="confirm_password" name="confirm_password" type="password" v-model="form.confirmPassword" placeholder="******************" class="shadow appearance-none w-full py-2 px-3 text-grey-darker mb-3 leading-tight focus:outline-none focus:shadow-outline">
                <p class="text-red text-xs italic invisible">Please choose a password.</p>
                </div>
                <div class="flex items-center justify-between">
                <button type="submit" class="bg-blue hover:bg-blue-dark text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline">
                    Sign In
                </button>
                <a class="inline-block align-baseline font-bold text-sm text-blue hover:text-blue-darker invisible" href="#">
                    Forgot Password?
                </a>
                </div>
            </form>
            <p class="text-center text-grey text-xs">
                ©2018
            </p>
        </div>
    </div>
</div>
</template>

<script>
// @ is an alias to /src
import URLprefix from "@/config.js";
import Navbar from "@/components/Navbar.vue";

export default {
  name: "Registration",
  components: {
    Navbar
  },
  data() {
    return {
      form: {
        username: "",
        email: "",
        password: "",
        confirmPassword: ""
      }
    };
  },
  methods: {
    signup() {
      fetch(URLprefix + "registration", {
        method: "POST",
        body: JSON.stringify(this.data),
        headers: {
          "content-type": "application/json",
          "Access-Control-Allow-Origin": "http://localhost:3030/"
        }
      })
        .then(response => response.json())
        .then(json => {
          window.location.reload(true);
          this.$router.push("/a/access");
        })
        .catch(e => {
          console.log(e);
        });
    }
  }
};
</script>
