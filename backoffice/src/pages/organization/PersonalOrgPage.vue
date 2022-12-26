<script lang="ts">
import OrgForm from 'src/components/organization/org-form.vue';
import useOrgsStore from 'src/stores/orgs';
import useUploadStore from 'src/stores/uploads';
import { computed, defineComponent, ref } from 'vue';
const uploadStore = useUploadStore();
const orgStore = useOrgsStore();
await orgStore.fetchCurrent();
export default defineComponent({
  name: 'PersonalOrg',
  components: { OrgForm },
  computed: {},
  async setup() {
    const logoFile = ref(null as unknown as File);
    const current = ref(orgStore.current);
    return {
      logoFile,
      current,
      title: computed(
        () =>
          `${current.value.name} ${
            current.value.vatNumber ? '(' + current.value.vatNumber + ')' : ''
          }`
      ),
    };
  },

  methods: {
    async update() {
      if (this.logoFile) {
        const upload = await uploadStore.uploadFile(
          this.logoFile,
          this.current.logoId,
          this.current._id
        );
        this.current.logoId = upload._id;
        this.logoFile = null as unknown as File;
      }
      this.current = await orgStore.update(this.current);
    },
    async reset(e: Event) {
      e.preventDefault();
      await orgStore.fetchCurrent();
      this.current = orgStore.current;
    },
  },
});
</script>

<template>
  <div class="row">
    <div class="col-12">
      <q-card v-if="current">
        <OrgForm
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
    </div>
  </div>
</template>

<style lang="sass" scoped></style>
