<script lang="ts">
import { Organization } from 'src/models/orgs';
import { computed, defineComponent } from 'vue';
export default defineComponent({
  name: 'OrgForm',
  props: {
    title: {
      type: String,
      default: () => 'Organization',
    },
    orgModel: {
      type: Object,
      default: () => ({} as Organization),
    },
    orgLogo: {
      type: Object,
      default: () => ({} as File),
    },
  },
  emits: ['update:orgModel', 'update:orgLogo'],
  async setup(props, context) {
    const org = computed({
      get: () => props.orgModel,
      set: (value) => context.emit('update:orgModel', value),
    });
    const logoFile = computed({
      get: () => props.orgLogo,
      set: (value) => context.emit('update:orgLogo', value),
    });
    return {
      org,
      logoFile,
    };
  },
});
</script>
<template>
  <div>My component</div>
</template>
