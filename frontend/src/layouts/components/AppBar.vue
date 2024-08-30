<template>
  <v-app-bar elevation="1">
    <v-app-bar-title>
      <v-avatar>
        <v-img src="https://public.svsticky.nl/logos/hoofd_outline_kleur.svg" alt="sticky-logo" />
      </v-avatar>
      {{ $t('site_title') }}
    </v-app-bar-title>

    <v-spacer/>

    <v-menu>
      <template v-slot:activator="{ props }">
        <v-btn
          class="mr-12"
          icon="mdi-web"
          v-bind="props"
        />
      </template>

      <v-list>
        <v-list-item
          v-for="locale in locales"
          :key="locale.locale"
          @click="setLocale(locale.locale)">
          <v-list-item-media>
            <v-img :src="locale.icon"></v-img>
          </v-list-item-media>
        </v-list-item>
      </v-list>
    </v-menu>
  </v-app-bar>
</template>

<script lang="ts">
import {defineComponent} from "vue";

export default defineComponent({
  data() {
    return {
      locales: [
        { icon: "https://flagicons.lipis.dev/flags/4x3/nl.svg", locale: "nl" },
        { icon: "https://flagicons.lipis.dev/flags/4x3/gb.svg", locale: "en" },
      ]
    }
  },
  methods: {
    setLocale(locale: string) {
      console.log(`Changing locale to ${locale}`);
      this.$root.$i18n.locale = locale;
    }
  }
});
</script>