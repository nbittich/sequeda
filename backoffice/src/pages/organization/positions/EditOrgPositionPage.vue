<script lang="ts">
import OrgPositionForm from 'src/components/organization/org-position-form.vue';
import useOrgPositionStore from 'src/stores/organization/position';
import { defineComponent, ref } from 'vue';

import { useRoute } from 'vue-router';
const positionStore = useOrgPositionStore();

export default defineComponent({
  name: 'NewOrgPositionPage',
  components: { OrgPositionForm },
  computed: {},
  async setup() {
    const route = useRoute();
    const positionId = route.params.id as string;
    const position = ref(await positionStore.findPositionById(positionId));
    return { position };
  },
  methods: {
    async update() {
      await positionStore.update(this.position);
      this.$router.push({ name: 'org.positions.root' });
    },
    async reset() {
      this.$router.push({ name: 'org.positions.root' });
    },
  },
});
</script>

<template>
  <div class="row">
    <div class="col-12">
      <q-card>
        <OrgPositionForm
          :title="'Edit Position'"
          v-model:position-model="position"
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
