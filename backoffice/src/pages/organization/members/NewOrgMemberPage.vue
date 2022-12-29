<script lang="ts">
import { Address } from 'cluster';
import OrgMemberForm from 'src/components/organization/org-member-form.vue';
import { ContactDetail } from 'src/models/contact-detail';
import { OrgMember, Remark } from 'src/models/orgs';
import { Person } from 'src/models/person';
import person from 'src/stores/person';
import { defineComponent, ref } from 'vue';

export default defineComponent({
  name: 'NewOrgMemberPage',
  components: { OrgMemberForm },
  computed: {},
  async setup() {
    const person = ref({
      contactDetail: { address: {} as Address } as ContactDetail,
    } as Person);
    const profilePictureFile = ref(null as unknown as File);
    const remarks = ref([] as Remark[]);
    const positionId = ref(null as unknown as string);
    const newMember = ref({
      responsibleOf: [],
    } as OrgMember);
    return { newMember, remarks, person, profilePictureFile, positionId };
  },
  methods: {},
});
</script>

<template>
  <div class="row">
    <div class="col-12">
      <OrgMemberForm
        :title="'New Member'"
        v-model:person-model="person"
        v-model:position-id-model="positionId"
        v-model:profile-picture-model="profilePictureFile"
        v-model:remarks-model="remarks"
      />
    </div>
  </div>
</template>

<style lang="sass" scoped></style>
