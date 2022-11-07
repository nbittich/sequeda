<template>
  {{ current }}
  <div class="row">
    <div class="col-md-8 col-12">
      <q-card class="my-card">
        <q-card-section>
          <div class="text-h6">Personal Information</div>
        </q-card-section>

        <q-card-section class="row">
          <div class="col-lg-4 col-12 q-mb-md">
            <q-input
              class="q-mr-xs q-mr-none"
              dense
              outlined
              v-model="current.firstName"
              label="First name"
            />
          </div>
          <div class="col-lg-4 col-12  q-mb-md">
            <q-input
              class="q-mr-xs q-mr-none"
              dense
              outlined
              v-model="current.middleName"
              label="Middle name"
            />
          </div>
          <div class="col-lg-4 col-12 q-mb-md">
            <q-input
              dense
              outlined
              v-model="current.lastName"
              label="Last name"
            />
          </div>
          <div class="col-12 q-mb-md">
            <q-input
            dense
            outlined
            label="Birth date"
            v-model="current.dateOfBirth"
            mask="date"
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
                  <q-date v-model="current.dateOfBirth">
                    <div class="row items-center justify-end">
                      <q-btn v-close-popup label="Close" color="primary" flat />
                    </div>
                  </q-date>
                </q-popup-proxy>
              </q-icon>
            </template>
          </q-input>
          </div>
        </q-card-section>


        <q-separator  />

        <q-card-actions>
          <q-btn flat>Save</q-btn>
          <q-btn flat>Cancel</q-btn>
        </q-card-actions>
      </q-card>
    </div>
  </div>
</template>

<script lang="ts">
import usePersonStore from 'src/stores/person';
import { defineComponent } from 'vue';
export default defineComponent({
  name: 'PersonalInformation',
  components: {},
  async setup() {
    const store = usePersonStore();
    await store.fetchCurrent();
    return { current: store.current };
  },
});
</script>
