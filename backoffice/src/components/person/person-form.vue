<script lang="ts">
import { computed, defineComponent, ref } from 'vue';
import {
  Person,
  genders,
  academicTitles,
  maritalStatuses,
} from 'src/models/person';

import ContactDetailForm from 'src/components/shared/contact-detail-form.vue';
import ImageUpload from 'src/components/shared/image-upload.vue';
import BankAccountForm from '../shared/bank-account-form.vue';
export default defineComponent({
  name: 'PersonForm',
  components: { ContactDetailForm, ImageUpload, BankAccountForm },
  props: {
    title: {
      type: String,
      default: () => 'Personal Information',
    },
    withSignature: {
      type: Boolean,
      default: () => false,
    },
    imageKey: {
      type: Number,
      default: () => ref(0),
    },
    personModel: {
      type: Object,
      default: () => ({}) as Person,
    },
    profilePicture: {
      type: Object,
      default: () => ({}) as File,
    },

    signature: {
      type: Object,
      default: () => ({}) as File,
    },
  },
  emits: ['update:personModel', 'update:profilePicture', 'update:signature'],
  async setup(props, context) {
    const person = computed({
      get: () => props.personModel,
      set: (value) => context.emit('update:personModel', value),
    });
    const profilePictureFile = computed({
      get: () => props.profilePicture,
      set: (value) => context.emit('update:profilePicture', value),
    });

    const signatureFile = computed({
      get: () => props.signature,
      set: (value) => context.emit('update:signature', value),
    });
    return {
      person,
      profilePictureFile,
      signatureFile,
      academicTitles,
      maritalStatuses,
      genders,
    };
  },
});
</script>
<template>
  <q-card>
    <q-card-section>
      <div class="text-h6">{{ title }}</div>
    </q-card-section>
    <q-card-section class="row justify-center">
      <div class="col">
        <div class="text-h6 q-ml-xs-sm">Picture Profile</div>
        <ImageUpload
          :key="imageKey"
          v-model="profilePictureFile"
          :picture-id="person.profilePictureId"
        />
      </div>
      <template v-if="withSignature">
        <div class="col">
          <div class="text-h6 q-ml-xs-sm">Signature</div>

          <ImageUpload
            :key="imageKey"
            v-model="signatureFile"
            :picture-id="person.signatureId"
          />
        </div>
      </template>
    </q-card-section>
    <q-separator />

    <q-card-section class="q-mb-none q-pb-none">
      <div class="row q-mb-xs-none q-mb-md-xs">
        <div class="col-lg-6 col-12 q-mb-xs-sm q-mb-lg-none">
          <q-input
            :autofocus="true"
            class="q-mr-sm-xs"
            dense
            outlined
            v-model="person.firstName"
            label="First name"
          />
        </div>
        <div class="col-lg-6 col-12 q-mb-xs-sm q-mb-lg-none">
          <q-input
            class="q-mr-md-xs"
            dense
            outlined
            v-model="person.lastName"
            label="Last name"
          />
        </div>
      </div>
      <div class="row q-mb-xs-none q-mb-md-xs">
        <div class="col-lg-6 col-12 q-mb-xs-sm q-mb-lg-none">
          <q-input
            class="q-mr-md-xs"
            dense
            outlined
            v-model="person.nickName"
            label="Nick name"
          />
        </div>
        <div class="col-lg-6 col-12 q-mb-xs-sm q-mb-lg-none">
          <q-input
            dense
            class="q-mr-md-xs"
            outlined
            v-model="person.middleName"
            label="Middle name"
          />
        </div>
      </div>

      <div class="row q-mb-xs-none q-mb-md-xs">
        <div class="col-lg-6 col-12">
          <q-input
            dense
            outlined
            class="q-mr-md-xs"
            label="Birth date"
            v-model="person.dateOfBirth"
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
                  <q-date mask="YYYY-MM-DD" v-model="person.dateOfBirth">
                    <div class="row items-center justify-end">
                      <q-btn v-close-popup label="Close" color="primary" flat />
                    </div>
                  </q-date>
                </q-popup-proxy>
              </q-icon>
            </template>
          </q-input>
        </div>
        <div class="col-lg-6 col-12 q-mb-xs-sm q-mb-lg-none">
          <q-select
            dense
            class="q-mr-md-xs"
            outlined
            v-model="person.gender"
            :options="genders"
            option-label="label"
            option-value="value"
            emit-value
            map-options
            label="Gender"
          />
        </div>
      </div>
    </q-card-section>

    <q-card-section class="q-mt-xs-sm q-mt-md-none q-pt-none">
      <div class="row q-mb-xs-none q-mb-md-xs">
        <div class="col-lg-6 col-12 q-mb-xs-sm q-mb-lg-none">
          <q-select
            class="q-mr-md-xs"
            dense
            outlined
            v-model="person.maritalStatus"
            :options="maritalStatuses"
            label="Status"
            option-label="label"
            option-value="value"
            emit-value
            map-options
          />
        </div>

        <div class="col-lg-6 col-12">
          <q-select
            dense
            class="q-mr-md-xs"
            outlined
            v-model="person.academicTitle"
            :options="academicTitles"
            label="Title"
            option-label="label"
            option-value="value"
            emit-value
            map-options
          />
        </div>
      </div>
    </q-card-section>

    <ContactDetailForm v-model="person.contactDetail" :title="'Contact'" />
    <BankAccountForm v-model="person.bankAccount"></BankAccountForm>
  </q-card>
</template>
