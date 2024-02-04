<script lang="ts">
import OrgForm from 'src/components/organization/org-form.vue';
import PersonForm from 'src/components/person/person-form.vue';
import RemarkForm from 'src/components/shared/remark-form.vue';
import { BankAccount } from 'src/models/bank-account';
import { Address, ContactDetail } from 'src/models/contact-detail';
import {
  Communication,
  Organization,
  OrgCustomer,
  representedByIsOrg,
} from 'src/models/orgs';
import { Person } from 'src/models/person';
import useCustomerStore from 'src/stores/organization/customer';
import useOrgsStore from 'src/stores/organization/orgs';
import usePersonStore from 'src/stores/person';
import useProductStore from 'src/stores/product';
import useUploadStore from 'src/stores/uploads';
import { defineComponent, ref } from 'vue';

const customerStore = useCustomerStore();
const uploadStore = useUploadStore();
const personStore = usePersonStore();
const orgStore = useOrgsStore();
const productStore = useProductStore();
const products = await productStore.findAll();
const currentOrg = await orgStore.fetchCurrent();

export default defineComponent({
  name: 'NewOrgCustomerPage',
  components: { OrgForm, PersonForm, RemarkForm },
  computed: {},
  async setup() {
    const representedByPerson = ref({
      contactDetail: { address: {} as Address } as ContactDetail,
      bankAccount: {} as BankAccount,
    } as Person);

    const representedByOrg = ref({
      primaryContact: { address: {} as Address } as ContactDetail,
      otherContacts: [] as ContactDetail[],
      primaryBankAccount: {} as BankAccount,
      otherBankAccounts: [] as BankAccount[],
      status: 'ACTIVE',
    } as Organization);
    const customerType = ref('PERSON');
    const recurringProducts = ref([] as string[]);
    const pictureFile = ref(null as unknown as File);
    const started = ref(null as unknown as string);
    const ended = ref(null as unknown as string);
    const communications = ref([] as Communication[]);
    return {
      communications,
      representedByPerson,
      representedByOrg,
      pictureFile,
      recurringProducts,
      customerType,
      products,
      started,
      ended,
    };
  },
  methods: {
    async update() {
      let representedBy: Person | Organization | null = null;
      if (this.customerType == 'ORGANIZATION') {
        representedBy = await orgStore.update(this.representedByOrg);
      } else {
        representedBy = await personStore.update(this.representedByPerson);
      }
      if (this.pictureFile) {
        const upload = await uploadStore.uploadFile(
          this.pictureFile,
          undefined,
          representedBy._id,
        );
        if (representedByIsOrg(representedBy)) {
          representedBy.logoId = upload._id;
          representedBy = await orgStore.update(representedBy);
        } else {
          representedBy.profilePictureId = upload._id;
          representedBy = await personStore.update(representedBy);
        }
        this.pictureFile = null as unknown as File;
      }
      let customer: OrgCustomer = {
        customerType: this.customerType,
        orgId: currentOrg._id,
        documentIds: [],
        started: this.started,
        recurringProductIds: this.recurringProducts,
        ended: this.ended,
        communications: this.communications,
        representedById: representedBy._id,
      };
      await customerStore.update(customer);
      this.$router.push({
        name: 'org.customers.root',
        query: { t: new Date().getTime() },
      });
    },
    async reset() {
      this.$router.push({ name: 'org.customers.root' });
    },
  },
});
</script>

<template>
  <div class="row">
    <div class="col-12">
      <q-card>
        <q-card-section>
          <div class="text-h6">New Customer</div>

          <div class="row">
            <div class="col-lg-6 col-12">
              <q-input
                dense
                outlined
                class="q-mr-md-xs"
                label="Started"
                v-model="started"
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
                      <q-date mask="YYYY-MM-DD" v-model="started">
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
            <div class="col-lg-6 col-12">
              <q-input
                dense
                outlined
                class="q-mr-md-xs"
                label="Ended"
                v-model="ended"
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
                      <q-date mask="YYYY-MM-DD" v-model="ended">
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
          </div>

          <div class="row">
            <div class="col-12">
              <q-select
                dense
                class="q-mr-md-xs"
                multiple
                use-chips
                use-input
                outlined
                v-model="recurringProducts"
                :options="products"
                option-label="label"
                option-value="_id"
                emit-value
                map-options
                label="Recurring products"
              />
            </div>
          </div>
        </q-card-section>
        <q-card-section>
          <q-toggle
            keep-color
            color="blue"
            v-model="customerType"
            true-value="PERSON"
            false-value="ORGANIZATION"
            :label="`Customer Type: ${
              customerType == 'PERSON' ? 'Person' : 'Organization'
            }`"
          />
        </q-card-section>
        <OrgForm
          v-if="customerType == 'ORGANIZATION'"
          v-model:orgModel="representedByOrg"
          v-model:orgLogo="pictureFile"
          title="Organization"
        />
        <PersonForm
          v-if="customerType == 'PERSON'"
          v-model:person-model="representedByPerson"
          v-model:profile-picture="pictureFile"
        />

        <RemarkForm v-model="communications" title="Communications" />

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
