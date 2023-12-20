<script lang="ts">
import { defineComponent, ref } from 'vue';
import useOrgPositionStore from 'src/stores/organization/position';
import { QTableColumn } from 'quasar';
const positionStore = useOrgPositionStore();
const columns: QTableColumn[] = [
  {
    name: 'level',
    align: 'left',
    label: 'Level',
    field: 'level',
    sortable: true,
  },
  {
    name: 'name',
    align: 'left',
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
  // {
  //   name: 'creationDate',
  //   align: 'center',
  //   label: 'Created',
  //   field: 'creationDate',
  //   sortable: false,
  // },
  // {
  //   name: 'updatedDate',
  //   align: 'center',
  //   label: 'Updated',
  //   field: 'updatedDate',
  //   sortable: false,
  // },
  {
    name: 'action',
    align: 'left',
    label: 'Action',
    field: 'action',
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
  methods: {
    newPosition() {
      this.$router.push({ name: 'org.positions.new' });
    },
  },
});
</script>

<template>
  <q-table dense :rows="positions" :columns="columns" row-key="_id">
    <template v-slot:top>
      <div class="row full-width justify-between">
        <div class="text-h6">Positions</div>
        <q-btn
          color="primary"
          icon="add"
          label="New position"
          @click="newPosition"
        />
      </div>
    </template>
    <template v-slot:body="props">
      <q-tr :props="props">
        <q-td key="level" :props="props">
          {{ props.row.level || 'N/A' }}
        </q-td>
        <q-td key="name" :props="props">
          {{ props.row.name }}
        </q-td>
        <q-td key="description" :props="props">
          {{ props.row.description?.substring(0, 100) }}...
        </q-td>
        <q-td key="action" :props="props">
          <q-btn
            round
            icon="edit"
            color="primary"
            :to="'/org/positions/' + props.row._id + '/edit'"
          ></q-btn>
        </q-td>
      </q-tr>
    </template>
  </q-table>
</template>

<style lang="sass" scoped></style>
