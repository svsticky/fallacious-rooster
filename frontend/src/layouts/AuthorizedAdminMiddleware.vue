<template>
  <v-app>
    <v-main v-if="error != null">
      <v-alert
        v-if="error != null"
        icon="mdi-alert-circle-outline"
        title="Login failed"
        :text="error"
        type="error"
      />
    </v-main>

    <AppBar
      v-if="loginOk"
      :is-admin="isAdmin"
    />
    <RouterView v-if="loginOk" />
  </v-app>
</template>

<script lang="ts">

import {defineComponent} from "vue";
import RouterView from "./components/RouterView.vue";
import AppBar from "./components/AppBar.vue";
import {checkLogin} from "@/layouts/authorized";

interface Data {
  error: string | null,
  loginOk: boolean,
  isAdmin: boolean,
}

export default defineComponent({
  components: {AppBar, RouterView},
  data(): Data {
    return {
      error: null,
      isAdmin: false,
      loginOk: false,
    }
  },
  async mounted() {
    const r = await checkLogin(true);
    if(r.isOk()) {
      const loginState = r.unwrap();
      this.loginOk = loginState.ok;
      this.isAdmin = loginState.isAdmin;

    } else {
      this.error = r.unwrapErr();
    }
  },
  methods: {

  }
})
</script>