<script lang="ts">
import useMemberStore from 'src/stores/organization/member';
import useOrgPositionStore from 'src/stores/organization/position';
import usePersonStore from 'src/stores/person';
import { defineComponent, ref } from 'vue';

const positionStore = useOrgPositionStore();
const memberStore = useMemberStore();
const personStore = usePersonStore();

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
