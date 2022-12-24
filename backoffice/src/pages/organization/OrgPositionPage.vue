<script lang="ts">
import { defineComponent, ref } from 'vue';
import useOrgPositionStore from 'src/stores/org_position';
import { QTableColumn } from 'quasar';
const positionStore = useOrgPositionStore();
const columns: QTableColumn[] = [
  {
    name: 'level',
    align: 'center',
    label: 'Level',
    field: 'level',
    sortable: true,
  },
  {
    name: 'name',
    align: 'center',
    label: 'Name',
    field: 'name',
    sortable: true,
  },
  {
    name: 'description',
    align: 'left',
    label: 'Description',
    field: 'description',
    sortable: false,
  },
  {
    name: 'creationDate',
    align: 'center',
    label: 'Created',
    field: 'creationDate',
    sortable: false,
  },
  {
    name: 'updatedDate',
    align: 'center',
    label: 'Updated',
    field: 'updatedDate',
    sortable: false,
  },
];

export default defineComponent({
  name: 'OrgPositions',
  components: {},
  computed: {},
  async setup() {
    const positions = ref(await positionStore.fetchPositions());

    return { columns, positions };
  },
  methods: {},
});
</script>

<template>
  <div class="row">
    <div class="col-12">
      <q-table
        title="Positions"
        dense
        :rows="positions"
        :columns="columns"
        row-key="_id"
      />
    </div>
  </div>
</template>

<style lang="sass" scoped></style>
