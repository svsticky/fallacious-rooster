<template>
  <v-container>
    <v-alert
      icon="mdi-alert-circle-outline"
      title="Error"
      :text=error
      @click:close="error = null"
      type="error"
      v-if="error != null"
    />

    <v-card>
      <v-card-title>{{ $t('home.welcome.title')}} </v-card-title>
      <v-card-text>
        <p>{{ $t('home.welcome.subtitle') }} </p>

        <h3 class="mt-4">{{ $t('home.form.title')}}</h3>
        <p> {{ $t('home.form.subtitle')}} </p>

        <v-form
          v-model="report.valid"
          itemref="reportForm">
          <v-textarea
            :label="$t('home.form.message')"
            v-model="report.message"
            auto-grow
            :rules="rules.required"
            rows="5"
            validate-on="blur"
            color="primary"
          />

          <v-row align="center">
            <v-col>
              <p class="mb-2">
                {{ $t('home.form.toBoard') }}
              </p>
            </v-col>
            <v-col>
              <v-checkbox
                v-model="report.toBoard"
                color="primary"
              />
            </v-col>
          </v-row>

          <v-row align="center">
            <v-col>
              <p class="mb-2">
                {{ $t('home.form.toAdvisors') }}:
              </p>
            </v-col>
            <v-col>
              <v-select
                v-model="report.toAdvisors"
                :label="$t('home.form.selectAdvisors')"
                :items="advisors"
                return-object
                multiple
                color="primary"
                item-title="name"
              />
            </v-col>
          </v-row>

          <v-row align="center">
            <v-col>
              <p class="mb-2">
                {{ $t('home.form.contactEmailExplanation') }}:
              </p>
            </v-col>
            <v-col>
              <v-text-field
                v-model="report.contactEmail"
                :rules="rules.optionalEmail"
                :label="$t('home.form.contactEmail')"
                validate-on="blur"
                color="primary"
              />
            </v-col>
          </v-row>

        </v-form>
      </v-card-text>

      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn
          variant="tonal"
          elevation="1"
          color="primary"
          @click="submitForm">
          {{ $t('home.form.submit') }}
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-container>
</template>
<script lang="ts">
import {defineComponent} from "vue";
import {ConfidentialAdvisor} from "@/scripts/config";
import {InputValidationRules} from "@/main";
import {VForm} from "vuetify/components/VForm";

interface Data {
  error: string | null,
  report: {
    valid: boolean,
    message: string | null,
    toBoard: boolean,
    toAdvisors: ConfidentialAdvisor[],
    contactEmail: string | null,
  },
  advisors: ConfidentialAdvisor[],
  rules: {
    required: InputValidationRules,
    optionalEmail: InputValidationRules,
  }
}

export default defineComponent({
  data(): Data {
    return {
      error: null,
      report: {
        valid: true,
        message: null,
        toBoard: false,
        toAdvisors: [],
        contactEmail: null,
      },
      advisors: [],
      rules: {
        required: [
          v => !!v || this.$t("home.form.required")
        ],
        optionalEmail: [
          v => v ? (/[^@ \t\r\n]+@[^@ \t\r\n]+\.[^@ \t\r\n]+/.test(v) || this.$t('home.form.invalidEmail')) : true
        ]
      }
    }
  },
  mounted() {
      this.loadAdvisors();
  },
  methods: {
    async loadAdvisors() {
      const r = await ConfidentialAdvisor.list();
      if(r.isErr()) {
        this.error = r.unwrapErr().message?? "Something went wrong";
        return;
      }

      this.advisors = r.unwrap();
    },
    async submitForm() {
      let form = this.$refs.reportForm as VForm;

      if(!this.report.valid || await form.validate()) {
        this.error = this.$t("home.form.invalid");
        return;
      }


    }
  }
})
</script>