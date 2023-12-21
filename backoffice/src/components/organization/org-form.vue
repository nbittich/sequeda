<script lang="ts">
import { BankAccount } from 'src/models/bank-account';
import { Address, ContactDetail } from 'src/models/contact-detail';
import { Organization, orgStatuses } from 'src/models/orgs';
import { computed, defineComponent, ref } from 'vue';
import BankAccountForm from '../shared/bank-account-form.vue';
import ContactDetailForm from '../shared/contact-detail-form.vue';
import ImageUpload from '../shared/image-upload.vue';
export default defineComponent({
  name: 'OrgForm',
  props: {
    imageKey: {
      type: Number,
      default: () => ref(0),
    },
    title: {
      type: String,
      default: () => 'Organization',
    },
    orgModel: {
      type: Object,
      default: () => ({}) as Organization,
    },
    orgLogo: {
      type: Object,
      default: () => ({}) as File,
    },
  },
  emits: ['update:orgModel', 'update:orgLogo'],
  async setup(props, context) {
    const orgComputed = computed({
      get: () => props.orgModel,
      set: (value) => context.emit('update:orgModel', value),
    });

    const org = ref(orgComputed);

    if (!org.value.otherContacts) {
      org.value.otherContacts = [];
    }
    if (!org.value.otherBankAccounts) {
      org.value.otherBankAccounts = [];
    }

    const orgClosed = ref(org.value.closedDate !== null);
    const logoFile = computed({
      get: () => props.orgLogo,
      set: (value) => context.emit('update:orgLogo', value),
    });
    return {
      org,
      orgStatuses,
      logoFile,
      orgClosed,
      deleteContact(index: number) {
        org.value.otherContacts.splice(index, 1);
      },
      addContact() {
        org.value.otherContacts.push({
          address: {} as Address,
        } as ContactDetail);
      },
      deleteBankAccount(index: number) {
        org.value.otherBankAccounts.splice(index, 1);
      },
      addBankAccount() {
        org.value.otherBankAccounts.push({} as BankAccount);
      },
    };
  },
  methods: {
    setStatus(val: string) {
      this.org.status = val;
      this.orgClosed = val !== 'ACTIVE';
      if (!this.orgClosed) {
        this.org.closedDate = null;
      }
    },
  },
  components: { ImageUpload, ContactDetailForm, BankAccountForm },
});
</script>
<template>
  <q-card>
    <q-card-section>
      <div class="text-h6">{{ title }}</div>
    </q-card-section>

    <q-card-section class="q-mb-none q-pb-none column items-center">
      <ImageUpload :key="imageKey" v-model="logoFile" :pictureId="org.logoId" />
    </q-card-section>
    <q-card-section class="q-mb-none q-pb-none">
      <div class="row q-mb-xs-none q-mb-md-xs">
        <div class="col-lg-6 col-12 q-mb-xs-sm q-mb-lg-none">
          <q-input
            :autofocus="true"
            class="q-mr-sm-xs"
            dense
            outlined
            v-model="org.name"
            label="Name"
          />
        </div>
        <div class="col-lg-6 col-12 q-mb-xs-sm q-mb-lg-none">
          <q-input
            :autofocus="true"
            class="q-mr-sm-xs"
            dense
            outlined
            v-model="org.vatNumber"
            label="VAT"
          />
        </div>
      </div>
      <div class="row q-mb-xs-none q-mb-md-xs">
        <div class="col-12">
          <q-input
            dense
            outlined
            class="q-mr-md-xs"
            label="Founded date"
            v-model="org.foundedDate"
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
                  <q-date mask="YYYY-MM-DD" v-model="org.foundedDate">
                    <div class="row items-center justify-end">
                      <q-btn v-close-popup label="Close" color="primary" flat />
                    </div>
                  </q-date>
                </q-popup-proxy>
              </q-icon>
            </template>
          </q-input>
        </div>
      </div>
      <div class="row q-mb-xs-none q-mb-md-xs">
        <div class="col-lg-6 col-12 q-mb-xs-sm q-mb-lg-none">
          <q-select
            dense
            class="q-mr-md-xs"
            outlined
            v-model="org.status"
            :options="orgStatuses"
            option-label="label"
            option-value="value"
            emit-value
            map-options
            label="Status"
            @update:model-value="setStatus"
          />
        </div>
        <div class="col-lg-6 col-12 q-mb-xs-sm q-mb-lg-none">
          <q-input
            dense
            outlined
            :disable="!orgClosed"
            class="q-mr-md-xs"
            label="Closed date"
            v-model="org.closedDate"
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
                  <q-date mask="YYYY-MM-DD" v-model="org.closedDate">
                    <div class="row items-center justify-end">
                      <q-btn v-close-popup label="Close" color="primary" flat />
                    </div>
                  </q-date>
                </q-popup-proxy>
              </q-icon>
            </template>
          </q-input>
        </div>
      </div>

      <div class="row q-mb-xs-none q-mb-md-xs">
        <div class="col-12">
          <q-input
            v-model="org.description"
            dense
            outlined
            label="Description"
            autogrow
          />
        </div>
      </div>
    </q-card-section>

    <ContactDetailForm
      v-model="org.primaryContact"
      :title="'Primary Contact'"
    />

    <div class="row justify-between q-pa-md q-gutter-sm">
      <div class="text-h6">Other Contacts</div>
      <q-btn round icon="add" color="primary" @click="addContact()" />
    </div>
    <template v-for="(oc, index) in org.otherContacts" :key="index">
      <ContactDetailForm
        v-if="oc"
        v-model="org.otherContacts[index]"
        :title="'Other Contact #' + (index + 1)"
        :deletable="true"
        @deleted="deleteContact(index)"
      />
    </template>
    <BankAccountForm
      v-model="org.primaryBankAccount"
      :title="'Primary Bank Account'"
    />

    <div class="row justify-between q-pa-md q-gutter-sm">
      <div class="text-h6">Other Bank Accounts</div>
      <q-btn round icon="add" color="primary" @click="addBankAccount()" />
    </div>
    <template v-for="(ob, index) in org.otherBankAccounts" :key="index">
      <BankAccountForm
        v-if="ob"
        v-model="org.otherBankAccounts[index]"
        :title="'Other Bank Account #' + (index + 1)"
        :deletable="true"
        @deleted="deleteBankAccount(index)"
      />
    </template>
  </q-card>
</template>
