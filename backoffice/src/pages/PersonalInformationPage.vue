<script lang="ts">
import { computed, defineComponent, ref } from 'vue';
import useUploadStore from 'src/stores/uploads';
import usePersonStore from 'src/stores/person';
import PersonForm from 'src/components/person/person-form.vue';

const personStore = usePersonStore();
const uploadStore = useUploadStore();
await personStore.fetchCurrent();

const imageKey = ref(0);
const reload = () => {
  imageKey.value += 1;
};
const current = ref(personStore.current);
const profilePictureFile = ref(null as unknown as File);
export default defineComponent({
  name: 'PersonalInformation',
  components: { PersonForm },

  async setup() {
    return {
      tab: ref('general'), // in case adding more tabs, see PersonalOrgPage.vue for an example
      imageKey,
      current,
      profilePictureFile,
      title: computed(
        () => `${current.value.firstName} ${current.value.lastName}`,
      ),
    };
  },
  methods: {
    async update() {
      if (this.profilePictureFile) {
        const upload = await uploadStore.uploadFile(
          this.profilePictureFile,
          this.current.profilePictureId,
          this.current._id,
        );
        this.current.profilePictureId = upload._id;
        this.profilePictureFile = null as unknown as File;
      }
      this.current = await personStore.update(this.current);
      reload();
    },
    async reset(e: Event) {
      e.preventDefault();
      await personStore.fetchCurrent();
      this.current = personStore.current;
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
    <q-route-tab
      to="/personal-info"
      name="general"
      icon="perm_identity"
      label="General"
    />
  </q-tabs>
  <q-separator />

  <q-tab-panels v-model="tab" v-if="current" animated>
    <q-tab-panel name="general">
      <q-card>
        <PersonForm
          :image-key="imageKey"
          v-model:personModel="current"
          v-model:profilePicture="profilePictureFile"
          :title="title"
        />
        <q-separator />
        <q-card-actions>
          <q-btn color="primary" @click="update">Save</q-btn>
          <q-btn color="deep-orange" @click="reset">Cancel</q-btn>
        </q-card-actions>
      </q-card>
    </q-tab-panel>
  </q-tab-panels>
</template>

<style lang="sass" scoped></style>
