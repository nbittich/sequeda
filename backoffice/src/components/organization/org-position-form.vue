<script lang="ts">
import { Position, positionLevel } from 'src/models/orgs';
import { computed, defineComponent, ref } from 'vue';

export default defineComponent({
  name: 'OrgPositionForm',
  props: {
    title: {
      type: String,
      default: () => '',
    },
    positionModel: {
      type: Object,
      default: () => ({}) as Position,
    },
  },
  emits: ['update:positionModel'],
  async setup(props, context) {
    const positionComputed = computed({
      get: () => props.positionModel,
      set: (value) => context.emit('update:positionModel', value),
    });
    const position = ref(positionComputed);

    return {
      position,
      positionLevel,
    };
  },
  methods: {},
  components: {},
});
</script>
<template>
  <q-card>
    <q-card-section>
      <div class="row justify-between">
        <div class="text-h6">{{ title }}</div>
      </div>
    </q-card-section>

    <q-card-section>
      <div class="row q-mb-xs-none q-mb-md-xs">
        <div class="col-12">
          <q-select
            class="q-mr-md-xs"
            dense
            outlined
            v-model="position.level"
            :options="positionLevel"
            option-label="label"
            option-value="value"
            emit-value
            map-options
            label="Level"
          >
          </q-select>
        </div>
      </div>
      <div class="row q-mb-xs-none q-mb-md-xs">
        <div class="col-12">
          <q-input
            dense
            outlined
            class="q-mr-md-xs"
            label="Name"
            v-model="position.name"
          >
          </q-input>
        </div>
      </div>
      <div class="row q-mb-xs-none q-mb-md-xs">
        <div class="col-12">
          <q-input
            dense
            outlined
            type="textarea"
            class="q-mr-md-xs"
            label="description"
            v-model="position.description"
          >
          </q-input>
        </div>
      </div>
    </q-card-section>
  </q-card>
</template>
