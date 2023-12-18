<script lang="ts">
import OrgForm from 'src/components/organization/org-form.vue';
import PersonForm from 'src/components/person/person-form.vue';
import RemarkForm from 'src/components/shared/remark-form.vue';
import { RepresentedBy, representedByIsOrg } from 'src/models/orgs';
import useCustomerStore from 'src/stores/organization/customer';
import useOrgsStore from 'src/stores/organization/orgs';
import usePersonStore from 'src/stores/person';
import useUploadStore from 'src/stores/uploads';
import { defineComponent, ref } from 'vue';
import { useRoute } from 'vue-router';

const customerStore = useCustomerStore();
const uploadStore = useUploadStore();
const personStore = usePersonStore();
const orgStore = useOrgsStore();

export default defineComponent({
  name: 'EditOrgCustomerPage',
  components: { OrgForm, PersonForm, RemarkForm },
  computed: {},
  async setup() {
    const route = useRoute();

    const customerId = route.params.id as string;
    const customer = ref(await customerStore.findOne(customerId));

    const representedBy = ref(null as unknown as RepresentedBy);
    if (customer.value.customerType == 'PERSON') {
      representedBy.value = await personStore.findOne(
        customer.value.representedById || '',
      );
    } else {
      representedBy.value = await orgStore.findOne(
        customer.value.representedById || '',
      );
    }
    const customerType = ref(customer.value.customerType || 'PERSON');

    const pictureFile = ref(null as unknown as File);
    return { representedBy, pictureFile, customer, customerType };
  },
  methods: {
    async update() {
      if (this.pictureFile) {
        const representedByPictureId = representedByIsOrg(this.representedBy)
          ? this.representedBy.logoId
          : this.representedBy.profilePictureId;
        const upload = await uploadStore.uploadFile(
          this.pictureFile,
          representedByPictureId,
          this.representedBy._id,
        );
        if (representedByIsOrg(this.representedBy)) {
          this.representedBy.logoId = upload._id;
          this.representedBy = await orgStore.update(this.representedBy);
        } else {
          this.representedBy.profilePictureId = upload._id;
          this.representedBy = await personStore.update(this.representedBy);
        }
        this.pictureFile = null as unknown as File;
      }

      await customerStore.update(this.customer);
      this.$router.push({ name: 'org.customers.root' });
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
          <div class="text-h6">Edit customer</div>
        </q-card-section>
        <q-card-section>
          <div class="row q-mb-xs-none q-mb-md-xs">
            <div class="col-lg-6 col-12">
              <q-input
                dense
                outlined
                class="q-mr-md-xs"
                label="Started"
                v-model="customer.started"
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
                      <q-date mask="YYYY-MM-DD" v-model="customer.started">
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
                v-model="customer.ended"
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
                      <q-date mask="YYYY-MM-DD" v-model="customer.ended">
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
        </q-card-section>
        <OrgForm
          v-if="customerType == 'ORGANIZATION'"
          v-model:orgModel="representedBy"
          v-model:orgLogo="pictureFile"
          title="Organization"
        />
        <PersonForm
          v-if="customerType == 'PERSON'"
          :title="'Person'"
          v-model:person-model="representedBy"
          v-model:profile-picture="pictureFile"
        />

        <RemarkForm v-model="customer.communications" title="Communications" />

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
