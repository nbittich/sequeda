<script lang="ts">
import usePersonStore from 'src/stores/person';
import { defineComponent, ref } from 'vue';
import useUploadStore from 'src/stores/uploads';
import ContactDetailForm from 'src/components/contact-detail-form.vue';
import ImageUpload from 'src/components/image-upload.vue';
const personStore = usePersonStore();
const uploadStore = useUploadStore();
await personStore.fetchCurrent();
export default defineComponent({
  name: 'PersonalInformation',
  components: { ContactDetailForm, ImageUpload },
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
    maritalStatuses: () => {
      return [
        {
          label: '-',
          value: null,
        },
        {
          label: 'Single',
          value: 'SINGLE',
        },
        {
          label: 'Married',
          value: 'MARRIED',
        },
        {
          label: 'Divorced',
          value: 'DIVORCED',
        },
        {
          label: 'Separated',
          value: 'SEPARATED',
        },
        {
          label: 'Civil Partnership',
          value: 'CIVIL_PARTNERSHIP',
        },
        {
          label: 'Widowed',
          value: 'WIDOWED',
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
    const profilePictureFile = ref(null as unknown as File);

    return {
      current,
      profilePictureFile,
    };
  },
  methods: {
    async update() {
      if (this.profilePictureFile) {
        const upload = await uploadStore.uploadFile(
          this.profilePictureFile,
          this.current.profilePictureId,
          this.current._id
        );
        this.current.profilePictureId = upload._id;
        this.profilePictureFile = null as unknown as File;
      }
      this.current = await personStore.update(this.current);
    },
    async reset(e: Event) {
      e.preventDefault();
      await personStore.fetchCurrent();
      this.current = personStore.current;
    },
  },
});
</script>

<template>
  <div class="row">
    <div class="col-12">
      <q-card v-if="current">
        <q-card-section>
          <div class="text-h6">Personal Information</div>
        </q-card-section>
        <q-card-section class="q-mb-none q-pb-none column items-center">
          <ImageUpload
            v-model="profilePictureFile"
            :pictureId="current.profilePictureId"
          />
        </q-card-section>
        <q-card-section class="q-mb-none q-pb-none">
          <div class="row q-mb-xs-none q-mb-md-xs">
            <div class="col-lg-6 col-12 q-mb-xs-sm q-mb-lg-none">
              <q-input
                :autofocus="true"
                class="q-mr-sm-xs"
                dense
                outlined
                v-model="current.firstName"
                label="First name"
              />
            </div>
            <div class="col-lg-6 col-12 q-mb-xs-sm q-mb-lg-none">
              <q-input
                class="q-mr-md-xs"
                dense
                outlined
                v-model="current.lastName"
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
                v-model="current.nickName"
                label="Nick name"
              />
            </div>
            <div class="col-lg-6 col-12 q-mb-xs-sm q-mb-lg-none">
              <q-input
                dense
                class="q-mr-md-xs"
                outlined
                v-model="current.middleName"
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
            <div class="col-lg-6 col-12 q-mb-xs-sm q-mb-lg-none">
              <q-select
                dense
                class="q-mr-md-xs"
                outlined
                v-model="current.gender"
                :options="gender"
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
                v-model="current.maritalStatus"
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
                v-model="current.academicTitle"
                :options="academicTitle"
                label="Title"
                option-label="label"
                option-value="value"
                emit-value
                map-options
              />
            </div>
          </div>
        </q-card-section>

        <ContactDetailForm v-model="current.contactDetail" :title="'Contact'" />

        <q-separator />

        <q-card-actions>
          <q-btn color="primary" @click="update">Save</q-btn>
          <q-btn color="deep-orange" @click="reset">Cancel</q-btn>
        </q-card-actions>
      </q-card>
    </div>
  </div>
</template>

<style lang="sass" scoped></style>
