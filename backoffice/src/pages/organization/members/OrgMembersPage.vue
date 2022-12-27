<script lang="ts">
import useMemberStore from 'src/stores/organization/member';
import useOrgPositionStore from 'src/stores/organization/position';
import { defineComponent, ref } from 'vue';

const positionStore = useOrgPositionStore();
const memberStore = useMemberStore();

export default defineComponent({
  name: 'OrgMembers',
  components: {},
  computed: {},
  async setup() {
    return {};
  },
  methods: {},
});
</script>

<template>
  <div class="row">
    <div class="col-12">
      <p>hello world!</p>
    </div>
  </div>
</template>

<style lang="sass" scoped></style>
