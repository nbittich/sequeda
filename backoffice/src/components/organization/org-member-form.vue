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
    startedModel: {
      type: String,
      default: () => null as unknown as string,
    },
    endedModel: {
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
    'update:startedModel',
    'update:endedModel',
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
    const startedComputed = computed({
      get: () => props.startedModel,
      set: (value) => context.emit('update:startedModel', value),
    });
    const endedComputed = computed({
      get: () => props.endedModel,
      set: (value) => context.emit('update:endedModel', value),
    });

    const person = ref(personComputed);
    const started = ref(startedComputed);
    const ended = ref(endedComputed);
    const positionId = ref(positionIdComputed);
    const remarks = ref(remarksComputed);

    const profilePictureFile = computed({
      get: () => props.profilePictureModel,
      set: (value) => context.emit('update:profilePictureModel', value),
    });
    return {
      person,
      remarks,
      started,
      ended,
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
      <div class="row q-mb-xs-none q-mb-md-xs">
        <div class="col-12">
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
        </div>
      </div>
      <div class="row q-mb-xs-none q-mb-md-xs">
        <div class="col-lg-6 col-12">
          <q-input
            dense
            outlined
            class="q-mr-md-xs"
            label="Started"
            v-model="started"
            :rules="['date']"
          >
            <template v-slot:append>
              <q-icon name="event" class="cursor-pointer">
                <q-popup-proxy
                  cover
                  :breakpoint="600"
                  transition-show="scale"
                  transition-hide="scale"
                >
                  <q-date mask="YYYY-MM-DD" v-model="started">
                    <div class="row items-center justify-end">
                      <q-btn v-close-popup label="Close" color="primary" flat />
                    </div>
                  </q-date>
                </q-popup-proxy>
              </q-icon>
            </template>
          </q-input>
        </div>
        <div class="col-lg-6 col-12">
          <q-input
            dense
            outlined
            class="q-mr-md-xs"
            label="Ended"
            v-model="ended"
            :rules="['date']"
          >
            <template v-slot:append>
              <q-icon name="event" class="cursor-pointer">
                <q-popup-proxy
                  cover
                  :breakpoint="600"
                  transition-show="scale"
                  transition-hide="scale"
                >
                  <q-date mask="YYYY-MM-DD" v-model="ended">
                    <div class="row items-center justify-end">
                      <q-btn v-close-popup label="Close" color="primary" flat />
                    </div>
                  </q-date>
                </q-popup-proxy>
              </q-icon>
            </template>
          </q-input>
        </div>
      </div>
    </q-card-section>

    <q-card-section>
      <div class="text-h6 q-mb-md">Remarks</div>
      <p>todo!</p>
    </q-card-section>
    <q-card-section>
      <div class="text-h6 q-mb-md">Managed by</div>
      <p>todo!</p>
    </q-card-section>
    <q-card-section>
      <div class="text-h6 q-mb-md">Manager of</div>
      <p>todo!</p>
    </q-card-section>
  </q-card>
</template>
