<script lang="ts">
import { Address } from 'cluster';
import OrgMemberForm from 'src/components/organization/org-member-form.vue';
import { BankAccount } from 'src/models/bank-account';
import { ContactDetail } from 'src/models/contact-detail';
import { OrgMember, Remark } from 'src/models/orgs';
import { Person } from 'src/models/person';
import useMemberStore from 'src/stores/organization/member';
import useOrgsStore from 'src/stores/organization/orgs';
import usePersonStore from 'src/stores/person';
import person from 'src/stores/person';
import useUploadStore from 'src/stores/uploads';
import { defineComponent, ref } from 'vue';

const memberStore = useMemberStore();
const uploadStore = useUploadStore();
const personStore = usePersonStore();
const orgStore = useOrgsStore();
const currentOrg = await orgStore.fetchCurrent();

export default defineComponent({
  name: 'NewOrgMemberPage',
  components: { OrgMemberForm },
  computed: {},
  async setup() {
    const person = ref({
      contactDetail: { address: {} as Address } as ContactDetail,
      bankAccount: {} as BankAccount,
    } as Person);
    const profilePictureFile = ref(null as unknown as File);
    const started = ref(null as unknown as string);
    const ended = ref(null as unknown as string);
    const remarks = ref([] as Remark[]);
    const positionId = ref(null as unknown as string);
    return { remarks, person, profilePictureFile, positionId, started, ended };
  },
  methods: {
    async update() {
      let person = await personStore.update(this.person);
      if (this.profilePictureFile) {
        const upload = await uploadStore.uploadFile(
          this.profilePictureFile,
          person.profilePictureId,
          person._id
        );
        person.profilePictureId = upload._id;
        this.profilePictureFile = null as unknown as File;
        person = await personStore.update(person);
      }
      let member: OrgMember = {
        orgId: currentOrg._id,
        started: this.started,
        ended: this.ended,
        personId: person._id,
        positionId: this.positionId,
        responsibleOf: [], // todo
        remarks: this.remarks,
      };
      await memberStore.update(member);
    },
    async reset() {
      alert('todo!');
    },
  },
});
</script>

<template>
  <div class="row">
    <div class="col-12">
      <q-card>
        <OrgMemberForm
          :title="'New Member'"
          v-model:person-model="person"
          v-model:position-id-model="positionId"
          v-model:profile-picture-model="profilePictureFile"
          v-model:remarks-model="remarks"
          v-model:started-model="started"
          v-model:ended-model="ended"
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
