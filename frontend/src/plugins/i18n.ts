import {createI18n} from "vue-i18n";
import EN from "@/plugins/locales/en";
import NL from "@/plugins/locales/nl";

export default createI18n({
  fallbackLocale: 'nl',
  messages: {
    en: EN,
    nl: NL
  }
})