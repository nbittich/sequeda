<script lang="ts">
import useGeoStore from 'src/stores/geoentities';
import { ContactDetail, PostalCode } from 'src/models/contact-detail';
import { computed, defineComponent, ref } from 'vue';

const geoStore = useGeoStore();
await geoStore.fetchCountries();

export default defineComponent({
  name: 'ContactDetailForm',
  props: {
    title: {
      type: String,
      default: () => 'Contact Detail',
    },
    deletable: {
      type: Boolean,
      default: () => false,
    },
    modelValue: {
      type: Object,
      default: () => ({} as ContactDetail),
    },
  },
  emits: ['update:modelValue', 'deleted'],
  async setup(props, context) {
    const contactDetail = computed({
      get: () => props.modelValue,
      set: (value) => context.emit('update:modelValue', value),
    });
    const countries = geoStore.countries;
    const countriesOptions = ref(geoStore.countries);
    const country = countries.find(
      (c) => c.label === contactDetail.value.address.country
    );
    const municipality = contactDetail.value.address.municipality;
    const selectedCountry = ref(country);
    const selectedPostalCode = ref({
      postalCode: contactDetail.value.address.postCode,
      countryCode: country?.code || '',
      name: municipality || '',
    } as PostalCode);
    const postalCodesOptions = ref(null as unknown as PostalCode[]);

    return {
      contactDetail,
      countriesOptions,
      selectedCountry,
      postalCodesOptions,
      selectedPostalCode,
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
        update: (arg0: () => void) => void
        // _abort: any
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
        update: (arg0: () => void) => void
        // _abort: any
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
            contactDetail.value.address.postCode = '';
            contactDetail.value.address.municipality = '';
          }
        }
      },
      setPostalCode(val: string) {
        if (val && selectedPostalCode.value) {
          contactDetail.value.address.municipality =
            selectedPostalCode.value.name;
          contactDetail.value.address.postCode =
            selectedPostalCode.value.postalCode;
        }
      },
    };
  },
  methods: {
    deleteContact() {
      if (this.deletable) {
        this.$emit('deleted');
      }
    },
  },
});
</script>

<template>
  <q-card>
    <q-card-section>
      <div class="row justify-between">
        <div class="text-h6">{{ title }}</div>
        <q-btn
          round
          icon="delete"
          color="red"
          v-if="deletable"
          @click="deleteContact"
        />
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
            v-model="contactDetail.emailAddress1"
            label="Email #1"
          />
        </div>
        <div class="col-lg-6 col-12 q-mb-xs-sm q-mb-lg-none">
          <q-input
            type="email"
            class="q-mr-md-xs"
            dense
            outlined
            v-model="contactDetail.emailAddress2"
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
            v-model="contactDetail.phoneNumber1"
            label="Gsm"
          />
        </div>
        <div class="col-lg-6 col-12 q-mb-xs-sm q-mb-lg-none">
          <q-input
            class="q-mr-md-xs"
            dense
            outlined
            v-model="contactDetail.phoneNumber2"
            label="Other Phone number"
          />
        </div>
      </div>
    </q-card-section>
    <q-card-section class="q-mt-xs-sm q-mt-md-none q-pt-none">
      <div class="row q-mb-xs-none q-mb-md-xs">
        <div class="col-lg-6 col-12 q-mb-xs-sm q-mb-lg-none">
          <q-input
            class="q-mr-md-xs"
            dense
            outlined
            v-model="contactDetail.address.street"
            label="Street"
          />
        </div>
        <div class="col-lg-1 col-12 q-mb-xs-sm q-mb-lg-none">
          <q-input
            class="q-mr-md-xs"
            dense
            outlined
            v-model="contactDetail.address.number"
            label="N°"
          />
        </div>

        <div class="col-lg-1 col-12 q-mb-xs-sm q-mb-lg-none">
          <q-input
            class="q-mr-md-xs"
            dense
            outlined
            v-model="contactDetail.address.boxNumber"
            label="Box"
          />
        </div>
        <div class="col-lg-4 col-12 q-mb-xs-sm q-mb-lg-none">
          <!-- TODO option-label & option-value why -->
          <q-select
            class="q-mr-md-xs"
            dense
            outlined
            v-model="contactDetail.address.country"
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
                <q-item-section class="text-grey"> No results </q-item-section>
              </q-item>
            </template>
          </q-select>
        </div>
      </div>
      <div class="row q-mb-xs-none q-mb-md-xs">
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
                <q-item-section class="text-grey"> No results </q-item-section>
              </q-item>
            </template>
          </q-select>
        </div>
        <div class="col-lg-5 col-12 q-mb-xs-sm q-mb-lg-none">
          <q-input
            class="q-mr-md-xs"
            :disable="true"
            dense
            outlined
            v-model="contactDetail.address.municipality"
            label="Municipality"
          />
        </div>
        <div class="col-lg-6 col-12 q-mb-xs-sm q-mb-lg-none">
          <q-input
            type="url"
            class="q-mr-md-xs"
            dense
            outlined
            v-model="contactDetail.website"
            label="Website"
          />
        </div>
      </div>
    </q-card-section>
  </q-card>
</template>
