<template>
  <v-container>
    <v-card>
      <v-card-title>
        <v-btn
          icon="mdi-arrow-left"
          to="/"
          variant="plain"
        ></v-btn>
        Settings
      </v-card-title>
      <v-card-text>
        <v-expansion-panels>
          <v-expansion-panel>
            <v-expansion-panel-title>Confidential Advisors</v-expansion-panel-title>
            <v-expansion-panel-text>
              <v-data-table
                :items="config.confidentialAdvisors"
                :headers="headers.confidentialAdvisors">
                <template v-slot:[`item.actions`]="{ item }">
                  <v-btn
                    icon="mdi-delete"
                    size="32"
                    color="primary"
                    @click="removeConfidentialAdvisor(item)"
                  />
                </template>
              </v-data-table>

              <h3>Add confidential advisor</h3>
              <v-form ref="newAdvisorForm" v-model="config.newAdvisor.valid">
                <v-row align="center" justify="space-between">
                  <v-col>
                    <v-text-field
                      label="Name"
                      v-model="config.newAdvisor.name"
                      :rules="rules.required"
                      color="primary"
                    />
                  </v-col>
                  <v-col>
                    <v-text-field
                      label="Email"
                      v-model="config.newAdvisor.email"
                      :rules="rules.email"
                      color="primary"
                    />
                  </v-col>
                  <v-col>
                    <v-btn
                      @click="addConfidentialAdvisor"
                      color="primary"
                      class="mb-5"
                      :loading="config.newAdvisor.loading"
                      :disabled="!config.newAdvisor.valid || config.newAdvisor.loading">
                      Save
                    </v-btn>
                  </v-col>
                </v-row>
              </v-form>
            </v-expansion-panel-text>
          </v-expansion-panel>
          <v-expansion-panel>
            <v-expansion-panel-title>
              Board
            </v-expansion-panel-title>
            <v-expansion-panel-text>
              <v-form v-model="config.board.valid">
                <v-row align="center">
                  <v-col>
                    Board email
                  </v-col>
                  <v-col>
                    <v-text-field
                      v-model="config.board.email"
                      :rules="rules.email"
                      color="primary"
                    />
                  </v-col>
                  <v-col>
                    <v-btn
                      icon="mdi-content-save-outline"
                      class="mb-4"
                      color="primary"
                      @click="updateBoardEmail"
                      :loading="config.board.loading"
                      :disabled="!config.board.valid && config.board.loading"
                    />
                  </v-col>
                </v-row>
              </v-form>
            </v-expansion-panel-text>
          </v-expansion-panel>
        </v-expansion-panels>
      </v-card-text>
    </v-card>
  </v-container>
</template>

<script lang="ts">

import {defineComponent} from "vue";
import {Board, ConfidentialAdvisor} from "@/scripts/config";
import {DataTableHeader, InputValidationRules} from "@/main";

interface Data {
  error: string | null,
  config: {
    confidentialAdvisors: ConfidentialAdvisor[],
    newAdvisor: {
      name: string | null,
      email: string | null,
      valid: boolean,
      loading: boolean,
    },
    board: {
      email: string | null,
      loading: boolean,
      valid: boolean,
    }
  },
  rules: {
    required: InputValidationRules,
    email: InputValidationRules,
  }
  headers: {
    confidentialAdvisors: DataTableHeader[],
  }
}

export default defineComponent({
  data(): Data {
    return {
      error: null,
      config: {
        confidentialAdvisors: [],
        newAdvisor: {
          name: null,
          email: null,
          valid: true,
          loading: false,
        },
        board: {
          email: null,
          valid: true,
          loading: false,
        }
      },
      headers: {
        confidentialAdvisors: [
          {
            title: "Name",
            value: "name"
          },
          {
            title: "Email",
            value: "email"
          },
          {
            title: "Actions",
            value: "actions"
          }
        ]
      },
      rules: {
        required: [
          v => !!v || "Required",
        ],
        email: [
          v => !!v || "Required",
          v => /[^@ \t\r\n]+@[^@ \t\r\n]+\.[^@ \t\r\n]+/.test(v) || "Invalid email address"
        ]
      }
    }
  },
  mounted() {
    this.loadAdvisors();
    this.loadBoardEmail();
  },
  methods: {
    async loadBoardEmail() {
      const r = await Board.getEmail();
      if(r.isOk()) {
        this.config.board.email = r.unwrap();
      } else {
        this.error = "Something went wrong";
      }
    },
    async updateBoardEmail() {
      this.config.board.loading = true;
      const r = await Board.updateEmail(this.config.board.email!);
      this.config.board.loading = false;

      if(r.isErr()) {
        this.error = "Something went wrong";
      }
    },
    async removeConfidentialAdvisor(advisor: ConfidentialAdvisor) {
      const r = await advisor.deleteAdvisor();
      if(r.isOk()) {
        await this.loadAdvisors();
      } else {
        this.error = "Something went wrong";
      }
    },
    async loadAdvisors() {
      const r = await ConfidentialAdvisor.list();
      if(r.isErr()) {
        this.error = r.unwrapErr().message?? "Something went wrong";
        return;
      }

      this.config.confidentialAdvisors = r.unwrap();
    },
    async addConfidentialAdvisor() {
      this.config.newAdvisor.loading = true;

      const r = await ConfidentialAdvisor.newAdvisor(
        this.config.newAdvisor.name!,
        this.config.newAdvisor.email!,
      );

      this.config.newAdvisor.loading = false;

      if(r.isOk()) {
        await this.loadAdvisors();
      } else {
        this.error = "Something went wrong"
      }
    }
  }
})
</script>