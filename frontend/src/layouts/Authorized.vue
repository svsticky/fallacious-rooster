<template>
    <v-app>
        <v-main v-if="error != null">
            <v-alert
                icon="mdi-alert-circle-outline"
                title="Login failed"
                :text=error
                type="error"
                v-if="error != null"
            />
        </v-main>

        <AppBar v-if="loginOk" :is-admin="isAdmin"/>
        <View v-if="loginOk" />
    </v-app>
</template>

<script lang="ts">

import {defineComponent} from "vue";
import View from "./components/View.vue";
import AppBar from "./components/AppBar.vue";
import {checkLogin} from "@/layouts/authorized";

interface Data {
  error: string | null,
  loginOk: boolean,
  isAdmin: boolean,
}

export default defineComponent({
    components: {AppBar, View},
    async mounted() {
        const r = await checkLogin();
        if(r.isOk()) {
          const loginState = r.unwrap();
          this.loginOk = loginState.ok;
          this.isAdmin = loginState.isAdmin;

        } else {
          this.error = r.unwrapErr();
        }
    },
    data(): Data {
        return {
          error: null,
          loginOk: false,
          isAdmin: false,
        }
    },
    methods: {

    }
})
</script>