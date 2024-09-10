<template>
  <v-container>
    <MaterialBanner
      :text="error"
      icon="mdi-alert-circle-outline"
      type="error"
      title="Error"
      @close="error = undefined"
    />

    <MaterialBanner
      title="Success"
      :text="success"
      type="success"
      icon="mdi-send-check"
      @close="success = undefined"
    />

    <v-card>
      <v-card-title>{{ $t('home.welcome.title') }} </v-card-title>
      <v-card-text>
        <p>{{ $t('home.welcome.subtitle') }} </p>

        <h3 class="mt-4">
          {{ $t('home.form.title') }}
        </h3>
        <p> {{ $t('home.form.subtitle') }} </p>

        <v-form v-model="report.valid">
          <v-textarea
            v-model="report.message"
            :label="$t('home.form.message')"
            auto-grow
            :rules="rules.required"
            rows="5"
            validate-on="blur"
            color="primary"
          />

          <v-row align="center">
            <v-col>
              <p class="mb-2">
                {{ $t('home.form.toReceivers') }}:
              </p>
            </v-col>
            <v-col>
              <v-select
                v-model="report.toReceivers"
                :label="$t('home.form.selectReceivers')"
                :items="receivers"
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
                {{ $t('home.form.allowContact') }}
              </p>
            </v-col>
            <v-col>
              <v-checkbox
                v-model="report.allowContact"
                color="primary"
              />
            </v-col>
          </v-row>

          <v-row
            v-if="report.allowContact"
            align="center"
          >
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
        <v-spacer />
        <v-btn
          variant="tonal"
          elevation="1"
          color="primary"
          @click="submitForm"
        >
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
import {Report} from "@/scripts/report"
import MaterialBanner from "@/views/components/MaterialBanner.vue"

interface Data {
  error: string | undefined,
  success: string | undefined,
  report: {
    valid: boolean,
    message: string | null,
    toReceivers: Receiver[],
    allowContact: boolean,
    contactEmail: string | null,
    loading: boolean,
  },
  receivers: Receiver[],
  rules: {
    required: InputValidationRules,
    optionalEmail: InputValidationRules,
  }
}

export class Receiver {
  name: string;
  inner: BoardReceiver | AdvisorReceiver;
  receiverType: ReceiverType;

  constructor(name: string, inner: BoardReceiver | AdvisorReceiver, receiverType: ReceiverType) {
    this.name = name;
    this.inner = inner;
    this.receiverType = receiverType;
  }
}

enum ReceiverType {
  BOARD,
  ADVISOR
}

class BoardReceiver {}

class AdvisorReceiver extends ConfidentialAdvisor {}

export default defineComponent({
  components: {MaterialBanner},
  data(): Data {
    return {
      error: undefined,
      success: undefined,
      report: {
        valid: true,
        message: null,
        toReceivers: [],
        allowContact: false,
        contactEmail: null,
        loading: false,
      },
      receivers: [],
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
        this.error = this.$t("error");
        return;
      }

      this.receivers = r.unwrap().map(advisor => new Receiver(advisor.name, advisor, ReceiverType.ADVISOR));
      this.receivers.push(new Receiver(this.$t('home.form.board'), new BoardReceiver(), ReceiverType.BOARD));
    },
    async submitForm() {
      if(!this.report.valid) {
        this.error = this.$t("home.form.invalid");
        return;
      }

      if(this.report.toReceivers.length == 0) {
        this.error = this.$t("home.form.selectRecipient");
        return;
      }

      const toAdvisors: AdvisorReceiver[] = this.report.toReceivers
        .filter(v => v.receiverType == ReceiverType.ADVISOR)
        .map(v => v.inner) as AdvisorReceiver[];
      const toBoard = this.report.toReceivers
        .filter(v => v.receiverType == ReceiverType.BOARD)
        .length > 0;

      this.report.loading = true;
      const r = await Report.report(this.report.message!, toBoard, toAdvisors, this.report.contactEmail);
      this.report.loading = false;

      if(r.isErr()) {
        this.error = this.$t("error");
      } else {
        this.success = this.$t("home.success");
      }
    }
  }
})
</script>