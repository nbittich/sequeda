<script lang="ts">
import { Person } from 'src/models/person';
import { computed, defineComponent, ref } from 'vue';
import PersonForm from '../person/person-form.vue';
import useOrgPositionStore from 'src/stores/organization/position';
import { Remark } from 'src/models/orgs';

const positionStore = useOrgPositionStore();

export default defineComponent({
  name: 'OrgMemberForm',
  props: {
    title: {
      type: String,
      default: () => 'Member',
    },
    personModel: {
      type: Object,
      default: () => ({} as Person),
    },
    remarksModel: {
      type: Object,
      default: () => [] as Remark[],
    },
    positionIdModel: {
      type: String,
      default: () => null as unknown as string,
    },
    profilePictureModel: {
      type: Object,
      default: () => ({} as File),
    },
  },
  emits: [
    'update:personModel',
    'update:profilePictureModel',
    'update:positionIdModel',
    'update:remarksModel',
  ],
  async setup(props, context) {
    const positions = await positionStore.fetchPositions();

    const positionsOptions = ref(positions);
    const personComputed = computed({
      get: () => props.personModel,
      set: (value) => context.emit('update:personModel', value),
    });
    const remarksComputed = computed({
      get: () => props.remarksModel,
      set: (value) => context.emit('update:remarksModel', value),
    });

    const positionIdComputed = computed({
      get: () => props.positionIdModel,
      set: (value) => context.emit('update:positionIdModel', value),
    });

    const person = ref(personComputed);
    const positionId = ref(positionIdComputed);
    const remarks = ref(remarksComputed);

    const profilePictureFile = computed({
      get: () => props.profilePictureModel,
      set: (value) => context.emit('update:profilePictureModel', value),
    });
    return {
      person,
      remarks,
      positionId,
      profilePictureFile,
      positions,
      positionsOptions,
      filterPosition(
        val: string,
        update: (arg0: () => void) => void
        // _abort: any
      ) {
        update(() => {
          const needle = val.toLocaleLowerCase();
          positionsOptions.value = positions.filter(
            (v) => v.name?.toLocaleLowerCase()?.indexOf(needle) > -1
          );
        });
      },
    };
  },
  methods: {},
  components: { PersonForm },
});
</script>
<template>
  <q-card>
    <q-card-section>
      <div class="text-h6">{{ title }}</div>
    </q-card-section>
    <PersonForm
      :title="'Person'"
      v-model:person-model="person"
      v-model:profile-picture="profilePictureFile"
    />
    <q-card-section>
      <div class="text-h6 q-mb-md">Position</div>

      <q-select
        class="q-mr-md-xs"
        dense
        outlined
        v-model="positionId"
        use-input
        option-label="name"
        option-value="_id"
        emit-value
        map-options
        hide-selected
        fill-input
        input-debounce="0"
        :options="positionsOptions"
        @filter="filterPosition"
        label="Position"
      >
        <template v-slot:no-option>
          <q-item>
            <q-item-section class="text-grey"> No results </q-item-section>
          </q-item>
        </template>
      </q-select>
    </q-card-section>
    <q-card-section>
      <div class="text-h6 q-mb-md">Remarks</div>
    </q-card-section>
  </q-card>
</template>
