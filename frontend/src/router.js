import Vue from "vue";
import Router from "vue-router";
import Home from "./views/Home.vue";
import Registration from "./views/Registration.vue";
import Authentication from "./views/Authentication.vue";

Vue.use(Router);

export default new Router({
  routes: [
    {
      path: "/",
      name: "home",
      component: Home
    },
    {
      path: "/registration",
      name: "registration",
      component: Registration
    },
    {
      path: "/authentication",
      name: "authentication",
      component: Authentication
    }
    // {
    //   path: "/about",
    //   name: "about",
    //   // route level code-splitting
    //   // this generates a separate chunk (about.[hash].js) for this route
    //   // which is lazy-loaded when the route is visited.
    //   component: () =>
    //     import(/* webpackChunkName: "about" */ "./views/About.vue")
    // }
  ]
});
