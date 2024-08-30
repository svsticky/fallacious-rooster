<template>
  <v-container>
    <v-card>
      <v-card-title>{{ $t('home.welcome.title')}} </v-card-title>
      <v-card-text>
        <p>{{ $t('home.welcome.subtitle') }} </p>

        <h3 class="mt-4">{{ $t('home.form.title')}}</h3>
        <p> {{ $t('home.form.subtitle')}} </p>

        <v-form>
          <v-textarea
            :label="$t('home.form.message')"
            v-model="report.message"
            auto-grow
            autofocus
            rows="5"
          />

        </v-form>
      </v-card-text>
    </v-card>
  </v-container>
</template>
<script lang="ts">
import {defineComponent} from "vue";

export default defineComponent({
  data() {
    return {
      report: {
        message: null
      }
    }
  }
})
</script>