<script lang="ts">
import OrgForm from 'src/components/organization/org-form.vue';
import useOrgsStore from 'src/stores/organization/orgs';
import useUploadStore from 'src/stores/uploads';
import { computed, defineComponent, ref } from 'vue';
import { useRoute } from 'vue-router';
const uploadStore = useUploadStore();
const orgStore = useOrgsStore();
await orgStore.fetchCurrent();

const imageKey = ref(0);
const reload = () => {
  imageKey.value += 1;
};
export default defineComponent({
  name: 'PersonalOrg',
  components: { OrgForm },
  computed: {},
  async setup() {
    const route = useRoute();
    const path = route.path;
    const tab = path.includes('members')
      ? 'members'
      : path.includes('positions')
        ? 'positions'
        : path.includes('customers')
          ? 'customers'
          : 'general';
    const logoFile = ref(null as unknown as File);
    const current = ref(orgStore.current);
    return {
      tab: ref(tab),
      logoFile,
      imageKey,
      current,
      title: computed(
        () =>
          `${current.value.name} ${
            current.value.vatNumber ? '(' + current.value.vatNumber + ')' : ''
          }`,
      ),
    };
  },

  methods: {
    async update() {
      if (this.logoFile) {
        const upload = await uploadStore.uploadFile(
          this.logoFile,
          this.current.logoId,
          this.current._id,
        );
        this.current.logoId = upload._id;
        this.logoFile = null as unknown as File;
      }
      this.current = await orgStore.update(this.current);
      reload();
    },
    async reset(e: Event) {
      e.preventDefault();
      await orgStore.fetchCurrent();
      this.logoFile = null as unknown as File;

      this.current = orgStore.current;
      reload();
    },
  },
});
</script>

<template>
  <q-tabs
    v-model="tab"
    class="text-teal"
    inline-label
    outside-arrows
    mobile-arrows
  >
    <q-route-tab to="/org" name="general" icon="store" label="General" />
    <q-route-tab
      to="/org/positions"
      name="positions"
      icon="school"
      label="Positions"
    />
    <q-route-tab
      to="/org/members"
      name="members"
      icon="badge"
      label="Members"
    />

    <q-route-tab
      to="/org/customers"
      name="customers"
      icon="recent_actors"
      label="Customers"
    />
  </q-tabs>
  <q-separator />
  <q-tab-panels v-model="tab" v-if="current" animated>
    <q-tab-panel name="general">
      <q-card>
        <OrgForm
          :image-key="imageKey"
          v-model:orgModel="current"
          v-model:orgLogo="logoFile"
          :title="title"
        />

        <q-separator />

        <q-card-actions>
          <q-btn color="primary" @click="update">Save</q-btn>
          <q-btn color="deep-orange" @click="reset">Cancel</q-btn>
        </q-card-actions>
      </q-card>
    </q-tab-panel>
    <q-tab-panel name="positions">
      <router-view :key="$route.params.id as string" />
    </q-tab-panel>

    <q-tab-panel name="customers">
      <router-view :key="$route.params.id as string" />
    </q-tab-panel>
    <q-tab-panel name="members">
      <router-view :key="$route.params.id as string" />
    </q-tab-panel>
  </q-tab-panels>
</template>

<style lang="sass" scoped></style>
