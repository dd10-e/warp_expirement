<template>
<div>
    <Navbar/>
    <div class="h-screen flex justify-center mt-4 lg:mt-32">
        <div class="w-full max-w-xs">
            <form action="http://localhost:3030/login" method="post" class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4">
                <div class="mb-4">
                <label for="username" class="block text-grey-darker text-sm font-bold mb-2">
                    Username
                </label>
                <input id="username" name="username" type="text" v-model="form.username" placeholder="Username" class="shadow appearance-none border rounded w-full py-2 px-3 text-grey-darker leading-tight focus:outline-none focus:shadow-outline">
                </div>
                <div class="mb-6">
                <label for="password" class="block text-grey-darker text-sm font-bold mb-2" >
                    Password
                </label>
                <input id="password" name="password" type="password" v-model="form.password" placeholder="******************" class="shadow appearance-none w-full py-2 px-3 text-grey-darker mb-3 leading-tight focus:outline-none focus:shadow-outline" >
                <p class="text-red text-xs italic invisible">Please choose a password.</p>
                </div>
                <div class="flex items-center justify-between">
                <button class="bg-blue hover:bg-blue-dark text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline" type="submit">
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
  name: "Authentication",
  components: {
    Navbar
  },
  data() {
    return {
      form: {
        username: "",
        password: ""
      }
    };
  },
  methods: {
    signin() {
      fetch(URLprefix + "user/signin", {
        body: JSON.stringify(data),
        headers: {
          "content-type": "application/json"
        },
        method: "POST"
      })
        .then(response => response.json())
        .then(json => {
          sessionStorage.setItem("token", json.token);
          sessionStorage.setItem(
            "signin_user",
            JSON.stringify(json.signin_user)
          );
          window.location.reload(true);
          this.$router.push("/");
        })
        .catch(e => {
          console.log(e);
        });
    }
  }
};
</script>
