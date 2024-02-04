<script lang="ts">
import OrgMemberForm from 'src/components/organization/org-member-form.vue';
import useMemberStore from 'src/stores/organization/member';
import usePersonStore from 'src/stores/person';
import useUploadStore from 'src/stores/uploads';
import { defineComponent, ref } from 'vue';
import { useRoute } from 'vue-router';

const memberStore = useMemberStore();
const uploadStore = useUploadStore();
const personStore = usePersonStore();

export default defineComponent({
  name: 'EditOrgMemberPage',
  components: { OrgMemberForm },
  computed: {},
  async setup() {
    const route = useRoute();

    const memberId = route.params.id as string;
    const member = ref(await memberStore.findOne(memberId));
    const person = ref(
      await personStore.findOne(member.value.personId as string),
    );
    const profilePictureFile = ref(null as unknown as File);
    const started = ref(member.value.started);
    const ended = ref(member.value.ended);
    const remarks = ref(member.value.remarks);
    const managedByIds = ref(member.value.managedBy);
    const responsibleOf = member.value.responsibleOf;

    const positionId = ref(member.value.positionId);

    return {
      remarks,
      person,
      profilePictureFile,
      positionId,
      started,
      ended,
      responsibleOf,
      member,
      managedByIds,
    };
  },

  methods: {
    async update() {
      let person = await personStore.update(this.person);
      if (this.profilePictureFile) {
        const upload = await uploadStore.uploadFile(
          this.profilePictureFile,
          person.profilePictureId,
          person._id,
        );
        person.profilePictureId = upload._id;
        this.profilePictureFile = null as unknown as File;
        person = await personStore.update(person);
      }
      this.member.started = this.started;
      this.member.ended = this.ended;
      this.member.positionId = this.positionId;
      this.member.remarks = this.remarks;

      this.member.managedBy = this.managedByIds;
      await memberStore.update(this.member);
      this.$router.push({
        name: 'org.members.root',
        query: { t: new Date().getTime() },
      });
    },
    async reset() {
      this.$router.push({ name: 'org.members.root' });
    },
  },
});
</script>

<template>
  <div class="row">
    <div class="col-12">
      <q-card>
        <OrgMemberForm
          :key="$route.path as string"
          :title="'Edit Member'"
          :responsible-of="responsibleOf"
          v-model:person-model="person"
          v-model:position-id-model="positionId"
          v-model:profile-picture-model="profilePictureFile"
          v-model:remarks-model="remarks"
          v-model:started-model="started"
          v-model:ended-model="ended"
          v-model:managed-by-ids-model="managedByIds"
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
