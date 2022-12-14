<script lang="ts">
import { computed, defineComponent, ref } from 'vue';
import useUploadStore from 'src/stores/uploads';
import usePersonStore from 'src/stores/person';
import PersonForm from 'src/components/person/person-form.vue';

const personStore = usePersonStore();
const uploadStore = useUploadStore();
await personStore.fetchCurrent();
export default defineComponent({
  name: 'PersonalInformation',
  components: { PersonForm },

  async setup() {
    const current = ref(personStore.current);
    const profilePictureFile = ref(null as unknown as File);
    return {
      current,
      profilePictureFile,
      title: computed(
        () => `${current.value.firstName} ${current.value.lastName}`
      ),
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
        <PersonForm
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
    </div>
  </div>
</template>

<style lang="sass" scoped></style>
