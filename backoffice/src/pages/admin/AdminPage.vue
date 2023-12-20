<script lang="ts">
import { defineComponent, ref } from 'vue';
import usePersonStore from 'src/stores/person';
import { useRoute } from 'vue-router';
const personStore = usePersonStore();
await personStore.fetchCurrent();
export default defineComponent({
  name: 'AdminPage',

  async setup() {
    const current = ref(personStore.current);
    const route = useRoute();
    const path = route.path;
    const tab = path.includes('logs') ? 'logs' : 'general';
    return {
      tab: ref(tab), // in case adding more tabs, see PersonalOrgPage.vue for an example
      current,
      title: 'Admin',
    };
  },
  methods: {},
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
      to="/admin"
      name="general"
      icon="admin_panel_settings"
      label="General"
    />
    <q-route-tab
      to="/admin/logs"
      name="logs"
      icon="history"
      label="Audit Logs"
    />
  </q-tabs>
  <q-separator />

  <q-tab-panels v-model="tab" v-if="current" animated>
    <q-tab-panel name="logs">
      <router-view :key="$route.params.id as string" />
    </q-tab-panel>
    <q-tab-panel name="general">
      <p>
        This is the admin panel. You can visualize logs, consult settings etc.
      </p>
    </q-tab-panel>
  </q-tab-panels>
</template>

<style lang="sass" scoped></style>
