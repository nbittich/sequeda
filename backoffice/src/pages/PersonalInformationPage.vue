
<script lang="ts">
import usePersonStore from 'src/stores/person';
import useGeoStore, { PostalCode } from 'src/stores/geoentities';
import { defineComponent, Ref, ref } from 'vue';
import useUploadStore from 'src/stores/uploads';
import { QFile } from 'quasar';
const personStore = usePersonStore();
const geoStore = useGeoStore();
const uploadStore = useUploadStore();
await personStore.fetchCurrent();
await geoStore.fetchCountries();
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
    const fileRef = ref() as Ref<QFile>;
    const profilePictureFile = ref(null as unknown as File);
    const current = ref(personStore.current);
    const countries = geoStore.countries;
    const countriesOptions = ref(geoStore.countries);
    const country = countries.find(
      (c) => c.label === personStore.current.contactDetail.address.country
    );
    const municipality = personStore.current.contactDetail.address.municipality;
    const selectedCountry = ref(country);
    const selectedPostalCode = ref({
      postalCode: personStore.current.contactDetail.address.postCode,
      countryCode: country?.code || '',
      name: municipality || '',
    } as PostalCode);
    const postalCodesOptions = ref(null as unknown as PostalCode[]);

    const profilePictureUrl = ref(null as unknown as string);
    const profilePictureUrlChange = async () => {
      console.log("ddd")
      if (profilePictureFile.value) {
           profilePictureUrl.value = URL.createObjectURL(profilePictureFile.value);
        } else {
          if (personStore.current.profilePictureId) {
            const pictureMetadata = await uploadStore.getMetadata(personStore.current.profilePictureId);
            profilePictureUrl.value = uploadStore.getDownloadUrl(pictureMetadata.thumbnailId);
          }else {
            profilePictureUrl.value = 'images/unknown.png';
          }
        }
    };
    await profilePictureUrlChange();

    return {
      current,
      fileRef,
      profilePictureFile,
      countriesOptions,
      selectedCountry,
      postalCodesOptions,
      selectedPostalCode,
      profilePictureUrl,
      profilePictureUrlChange,
      selectFile() {
        fileRef.value.pickFiles();
      },
      municipalityLabel(opt?: PostalCode | string) {
        if (!opt) {
          return '';
        }
        if (typeof opt === 'string') {
          return opt;
        }
        return selectedPostalCode.value === opt
          ? opt.postalCode
          : `${opt.postalCode} ${opt.name}`;
      },
      filterCountry(
        val: string,
        update: (arg0: () => void) => void,
        _abort: any
      ) {
        update(() => {
          const needle = val.toLocaleLowerCase();
          countriesOptions.value = countries.filter(
            (v) => v.label?.toLocaleLowerCase()?.indexOf(needle) > -1
          );
        });
      },
      async filterPostalCodes(
        val: string,
        update: (arg0: () => void) => void,
        _abort: any
      ) {
        let postCodes: PostalCode[] = [];
        if (selectedCountry.value) {
          postCodes = await geoStore.postCodesByQuery(
            selectedCountry.value,
            val.trim()
          );
        }
        update(() => {
          postalCodesOptions.value = postCodes;
        });
      },
      setCountry(val: string) {
        if (val) {
          selectedCountry.value = countries.find((c) => c.label === val);
          if (
            selectedPostalCode.value &&
            selectedPostalCode.value.countryCode !== selectedCountry.value?.code
          ) {
            selectedPostalCode.value = null as unknown as PostalCode;
            current.value.contactDetail.address.postCode = '';
            current.value.contactDetail.address.municipality = '';
          }
        }
      },
      setPostalCode(val: string) {
        if (val && selectedPostalCode.value) {
          current.value.contactDetail.address.municipality =
            selectedPostalCode.value.name;
          current.value.contactDetail.address.postCode =
            selectedPostalCode.value.postalCode;
        }
      },
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
      this.current =  personStore.current;
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
          <q-img
            class="border-fluid"
            :src="profilePictureUrl"
            spinner-color="white"
            @click="selectFile()"
            style="height: 140px; max-width: 150px"
          />
          <q-file
            ref="fileRef"
            style="display: none"
            v-model="profilePictureFile"
            @update:model-value="profilePictureUrlChange()"
            accept="image/*"
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
                outlined
                v-model="current.middleName"
                label="Middle name"
              />
            </div>
          </div>

          <div class="row q-mb-xs-none q-mb-md-xs">
            <div class="col-lg-4 col-12">
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
            <div class="col-lg-4 col-12 q-mb-xs-sm q-mb-lg-none">
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
                map-options
              />
            </div>
          </div>
        </q-card-section>

        <q-card-section class="q-mt-xs-sm q-mt-md-none q-pt-none">
          <div class="row q-mb-xs-none q-mb-md-xs">
            <div class="col-lg-4 col-12 q-mb-xs-sm q-mb-lg-none">
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
            <div class="col-lg-8 col-12 q-mb-xs-sm q-mb-lg-none">
              <q-input
                type="url"
                dense
                outlined
                v-model="current.contactDetail.website"
                label="Website"
              />
            </div>
          </div>
        </q-card-section>
        <q-card-section class="q-mt-xs-sm q-mt-md-none q-pt-none">
          <div class="row q-mb-xs-none q-mb-md-xs">
            <div class="col-lg-6 col-12 q-mb-xs-sm q-mb-lg-none">
              <q-input
                type="email"
                class="q-mr-md-xs"
                dense
                outlined
                v-model="current.contactDetail.emailAddress1"
                label="Email #1"
              />
            </div>
            <div class="col-lg-6 col-12 q-mb-xs-sm q-mb-lg-none">
              <q-input
                type="email"
                dense
                outlined
                v-model="current.contactDetail.emailAddress2"
                label="Email #2"
              />
            </div>
          </div>
          <div class="row q-mb-xs-none q-mb-md-xs">
            <div class="col-lg-6 col-12 q-mb-xs-sm q-mb-lg-none">
              <q-input
                class="q-mr-md-xs"
                dense
                outlined
                v-model="current.contactDetail.phoneNumber1"
                label="Gsm"
              />
            </div>
            <div class="col-lg-6 col-12 q-mb-xs-sm q-mb-lg-none">
              <q-input
                dense
                outlined
                v-model="current.contactDetail.phoneNumber2"
                label="Other Phone number"
              />
            </div>
          </div>
        </q-card-section>
        <q-card-section class="q-mt-xs-sm q-mt-md-none q-pt-none">
          <div class="row q-mb-xs-none q-mb-md-xs">
            <div class="col-lg-3 col-12 q-mb-xs-sm q-mb-lg-none">
              <q-input
                class="q-mr-md-xs"
                dense
                outlined
                v-model="current.contactDetail.address.street"
                label="Street"
              />
            </div>
            <div class="col-lg-1 col-12 q-mb-xs-sm q-mb-lg-none">
              <q-input
                class="q-mr-md-xs"
                dense
                outlined
                v-model="current.contactDetail.address.number"
                label="N°"
              />
            </div>

            <div class="col-lg-1 col-12 q-mb-xs-sm q-mb-lg-none">
              <q-input
                class="q-mr-md-xs"
                dense
                outlined
                v-model="current.contactDetail.address.boxNumber"
                label="Box"
              />
            </div>
            <div class="col-lg-3 col-12 q-mb-xs-sm q-mb-lg-none">
              <q-select
                class="q-mr-md-xs"
                dense
                outlined
                v-model="current.contactDetail.address.country"
                use-input
                option-label="label"
                option-value="label"
                emit-value
                map-options
                hide-selected
                fill-input
                input-debounce="0"
                :options="countriesOptions"
                @filter="filterCountry"
                @input-value="setCountry"
                label="Country"
              >
                <template v-slot:no-option>
                  <q-item>
                    <q-item-section class="text-grey">
                      No results
                    </q-item-section>
                  </q-item>
                </template>
              </q-select>
            </div>
            <div class="col-lg-1 col-12 q-mb-xs-sm q-mb-lg-none">
              <q-select
                class="q-mr-md-xs"
                dense
                outlined
                v-model="selectedPostalCode"
                use-input
                :option-label="municipalityLabel"
                map-options
                emit-value
                hide-selected
                fill-input
                input-debounce="0"
                :options="postalCodesOptions"
                @filter="filterPostalCodes"
                @input-value="setPostalCode"
                label="Post code"
              >
                <template v-slot:no-option>
                  <q-item>
                    <q-item-section class="text-grey">
                      No results
                    </q-item-section>
                  </q-item>
                </template>
              </q-select>
            </div>
            <div class="col-lg-3 col-12 q-mb-xs-sm q-mb-lg-none">
              <q-input
                :disable="true"
                dense
                outlined
                v-model="current.contactDetail.address.municipality"
                label="Municipality"
              />
            </div>
          </div>
        </q-card-section>
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
