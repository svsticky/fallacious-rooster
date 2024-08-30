<template>
    <v-app>
        <v-main v-if="loginErrorText != null">
            <v-alert
                icon="mdi-alert-circle-outline"
                title="Login failed"
                :text=loginErrorText
                type="error"
                v-if="loginErrorText != null"
            />
        </v-main>

        <AppBar v-if="loginOk" />
        <View v-if="loginOk" />
    </v-app>
</template>

<script lang="ts">

import {OAuth2Client} from "@/scripts/oauth2";
import {ApiError} from "@/scripts/error";
import {defineComponent} from "vue";
import View from "./components/View.vue";
import AppBar from "./components/AppBar.vue";

export default defineComponent({
    components: {AppBar, View},
    async mounted() {
        await this.checkLogin();
    },
    data() {
        return {
            loginErrorText: null,
            loginOk: false,
        }
    },
    methods: {
        async checkLogin() {
            const result = await OAuth2Client.ok();
            if(result.isOk()) {
                window.localStorage.setItem('koala-admin', true);
                this.loginOk = true;
            } else {
                const error: ApiError = result.unwrapErr();
                switch(error.status!) {
                    case 401: {
                        const loginLocation = await OAuth2Client.login();
                        if(loginLocation.isOk()) {
                            window.location.href = loginLocation.unwrap();
                        } else {
                            this.loginErrorText = "Could not log you in. Please try again later";
                            console.error(`Login redirect failed (${error.status}): ${error.message}`);
                        }

                        break;
                    }
                    case 500:
                    case 502: {
                        this.loginErrorText = "The server is having some troubles. Please try again later";
                        break;
                    }
                    default: {
                        console.error(`Login check failed (${error.status}): ${error.message}`);
                        this.loginErrorText = "Could not log you in. Please try again later";
                        break
                    }
                }
            }
        }
    }
})
</script>