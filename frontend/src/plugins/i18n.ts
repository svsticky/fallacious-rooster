/* eslint-disable @typescript-eslint/no-explicit-any */

import {createI18n} from "vue-i18n";
import EN from "@/plugins/locales/en";
import NL from "@/plugins/locales/nl";

function detectDefaultLocale(): string {
  if (window.location.hostname.startsWith('feelsafe')) {
    return 'en-US';
  }

  const browserLanguages = navigator.languages ?? [navigator.language];
  if (browserLanguages.some((lang) => lang.toLowerCase().startsWith('nl'))) {
    return 'nl';
  }

  return 'en-US';
}

export default createI18n({
  fallbackLocale: 'nl',
  locale: detectDefaultLocale(),
  messages: {
    "en-US": EN as any,
    nl: NL as any
  }
})

/* eslint-enable @typescript-eslint/no-explicit-any */