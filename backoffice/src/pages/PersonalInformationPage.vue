<template>
  <div class="row">
    <div class="col-12">
      <q-card v-if="current">
        <q-card-section>
          <div class="text-h6">Personal Information</div>
        </q-card-section>

        <q-card-section class="q-mb-none q-pb-none">
          <div class="row q-mb-xs-none q-mb-md-xs">
            <div class="col-lg-4 col-12 q-mb-xs-sm q-mb-md-none">
              <q-input
                :autofocus="true"
                class="q-mr-md-xs"
                dense
                outlined
                v-model="current.firstName"
                label="First name"
              />
            </div>
            <div class="col-lg-4 col-12 q-mb-xs-sm q-mb-md-none">
              <q-input
                class="q-mr-md-xs"
                dense
                outlined
                v-model="current.lastName"
                label="Last name"
              />
            </div>
            <div class="col-lg-4 col-12 q-mb-xs-sm q-mb-md-none">
              <q-input
                dense
                outlined
                v-model="current.middleName"
                label="Middle name"
              />
            </div>
          </div>
          <div class="row q-mb-xs-none q-mb-md-xs">
            <div class="col-md-4 col-12">
              <q-input
                dense
                outlined
                class="q-mr-md-xs"
                label="Birth date"
                v-model="current.dateOfBirth"
                :rules="['date']"
              >
                <template v-slot:append>
                  <q-icon name="event" class="cursor-pointer">
                    <q-popup-proxy
                      cover
                      :breakpoint="600"
                      transition-show="scale"
                      transition-hide="scale"
                    >
                      <q-date mask="YYYY-MM-DD" v-model="current.dateOfBirth">
                        <div class="row items-center justify-end">
                          <q-btn
                            v-close-popup
                            label="Close"
                            color="primary"
                            flat
                          />
                        </div>
                      </q-date>
                    </q-popup-proxy>
                  </q-icon>
                </template>
              </q-input>
            </div>
            <div class="col-lg-4 col-12 q-mb-xs-sm q-mb-md-none">
              <q-select
                dense
                class="q-mr-md-xs"
                outlined
                v-model="current.gender"
                :options="gender"
                option-label="label"
                option-value="value"
                emit-value
                label="Gender"
              />
            </div>
            <div class="col-lg-4 col-12">
              <q-select
                dense
                outlined
                v-model="current.academicTitle"
                :options="academicTitle"
                label="Title"
                option-label="label"
                option-value="value"
                emit-value
              />
            </div>
          </div>
        </q-card-section>

        <q-card-section class="q-mt-xs-sm q-mt-md-none q-pt-none">
          <div class="row q-mb-xs-none q-mb-md-xs">
            <div class="col-lg-6 col-12 q-mb-xs-sm q-mb-md-none">
              <q-input
                type="email"
                class="q-mr-md-xs"
                dense
                outlined
                v-model="current.contactDetail.emailAddress1"
                label="Email #1"
              />
            </div>
            <div class="col-lg-6 col-12 q-mb-xs-sm q-mb-md-none">
              <q-input
                type="email"
                class="q-mr-md-xs"
                dense
                outlined
                v-model="current.contactDetail.emailAddress2"
                label="Email #2"
              />
            </div>
          </div>
          <div class="row q-mb-xs-none q-mb-md-xs">
            <div class="col-lg-6 col-12 q-mb-xs-sm q-mb-md-none">
              <q-input
                class="q-mr-md-xs"
                dense
                outlined
                v-model="current.contactDetail.phoneNumber1"
                label="Gsm"
              />
            </div>
            <div class="col-lg-6 col-12 q-mb-xs-sm q-mb-md-none">
              <q-input
                class="q-mr-md-xs"
                dense
                outlined
                v-model="current.contactDetail.phoneNumber2"
                label="Other Phone number"
              />
            </div>
          </div>

          <div class="col-12 q-mb-xs-sm q-mb-md-none">
            <q-input
              type="url"
              class="q-mr-xs q-mr-none"
              dense
              outlined
              v-model="current.contactDetail.website"
              label="Website"
            />
          </div>
        </q-card-section>
        <q-card-section class="q-mt-xs-sm q-mt-md-none q-pt-none">
          <div class="row q-mb-xs-none q-mb-md-xs">
            <div class="col-lg-3 col-12 q-mb-xs-sm q-mb-md-none">
              <q-input
                class="q-mr-md-xs"
                dense
                outlined
                v-model="current.contactDetail.address.street"
                label="Street"
              />
            </div>
            <div class="col-lg-1 col-12 q-mb-xs-sm q-mb-md-none">
              <q-input
                class="q-mr-md-xs"
                dense
                outlined
                v-model="current.contactDetail.address.number"
                label="N°"
              />
            </div>

            <div class="col-lg-1 col-12 q-mb-xs-sm q-mb-md-none">
              <q-input
              class="q-mr-md-xs"

                dense
                outlined
                v-model="current.contactDetail.address.boxNumber"
                label="Box"
              />
            </div>

            <div class="col-lg-1 col-12 q-mb-xs-sm q-mb-md-none">
              <q-input
              class="q-mr-md-xs"

                dense
                outlined
                v-model="current.contactDetail.address.postCode"
                label="Post code"
              />
            </div>
            <div class="col-lg-3 col-12 q-mb-xs-sm q-mb-md-none">
              <q-input
              class="q-mr-md-xs"

                dense
                outlined
                v-model="current.contactDetail.address.municipality"
                label="Municipality"
              />
            </div>
            <div class="col-lg-3 col-12 q-mb-xs-sm q-mb-md-none">
              <q-input
              class="q-mr-md-xs"

                dense
                outlined
                v-model="current.contactDetail.address.country"
                label="Country"
              />
            </div>
          </div>
        </q-card-section>
        <q-separator />

        <q-card-actions>
          <q-btn flat @click="update">Save</q-btn>
          <q-btn flat @click="reset">Cancel</q-btn>
        </q-card-actions>
      </q-card>
    </div>
  </div>
</template>

<script lang="ts">
import usePersonStore from 'src/stores/person';

import { defineComponent, ref } from 'vue';
const personStore = usePersonStore();
await personStore.fetchCurrent();

export default defineComponent({
  name: 'PersonalInformation',
  components: {},
  computed: {
    gender: () => {
      return [
        {
          label: 'Male',
          value: 'MALE',
        },
        {
          label: 'Female',
          value: 'FEMALE',
        },
        {
          label: 'Unknown',
          value: 'UNKNOWN',
        },
      ];
    },
    academicTitle: () => {
      return [
        {
          label: '-',
          value: null,
        },
        {
          label: 'Doctor',
          value: 'DR',
        },
        {
          label: 'Professor',
          value: 'PROFESSOR',
        },
      ];
    },
  },
  async setup() {
    const current = ref(personStore.current);
    return { current };
  },
  methods: {
    async update() {
      this.current = await personStore.update(this.current);
    },
    async reset(e: Event) {
      e.preventDefault();
    }
  }
});
</script>
<style lang="sass" scoped></style>
