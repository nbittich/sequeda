<script lang="ts">
import { QTableColumn } from 'quasar';
import { OrgMemberDetail } from 'src/models/orgs';
import { Page, Pageable, PaginationProp } from 'src/models/pagination';
import useMemberStore from 'src/stores/organization/member';
import useOrgsStore from 'src/stores/organization/orgs';
import useOrgPositionStore from 'src/stores/organization/position';
import usePersonStore from 'src/stores/person';
import { defineComponent, ref } from 'vue';

const positionStore = useOrgPositionStore();
const memberStore = useMemberStore();
const personStore = usePersonStore();
const orgStore = useOrgsStore();

const currentOrg = await orgStore.fetchCurrent();

const positions = await positionStore.fetchPositions();

const columns: QTableColumn[] = [
  {
    name: 'name',
    align: 'left',
    label: 'Name',
    sortable: false,
    field: (row) => row.name,
  },
  {
    name: 'position',
    align: 'left',
    label: 'Position',
    field: (row) => row.position.name,
    sortable: false,
  },
  {
    name: 'started',
    align: 'left',
    label: 'Start Date',
    field: 'started',
    sortable: false,
  },
  {
    name: 'ended',
    align: 'left',
    label: 'End Date',
    field: 'ended',
    sortable: false,
  },
  {
    name: 'action',
    align: 'left',
    label: 'Action',
    field: 'action',
    sortable: false,
  },
];

export default defineComponent({
  name: 'OrgMembers',
  components: {},
  computed: {},
  async setup() {
    const pageRequest = ref({ limit: 10, page: 0 } as Pageable);
    const members = ref(null as unknown as Page<OrgMemberDetail>);
    const pagination = ref({ page: 1, rowsPerPage: 10 } as {
      page: number;
      rowsPerPage: number;
      rowsNumber: number;
    });
    const fetchPageMembers = async (props: PaginationProp) => {
      const { page, rowsPerPage } = props.pagination;
      pageRequest.value.page = (page || 1) - 1;
      pageRequest.value.limit = rowsPerPage || pageRequest.value.limit;
      members.value = await memberStore.fetchMembers(
        currentOrg._id as string,
        pageRequest.value,
      );
      if (members.value?.content?.length) {
        const personIds = members.value.content
          .filter((m) => typeof m.personId == 'string')
          .map((m) => m.personId as string);
        const persons = await personStore.findByIds(personIds);
        for (const member of members.value.content) {
          member.position = positions.find((p) => p._id === member.positionId);
          member.person = persons.find((p) => p._id == member.personId);
        }
      }
      pagination.value = {
        page: members.value.currentPage + 1,
        rowsPerPage: pageRequest.value.limit,
        rowsNumber: members.value.totalElements,
      };
    };
    await fetchPageMembers({ pagination: pagination.value });
    return {
      members,
      pageRequest,
      columns,
      fetchPageMembers,
      pagination,
    };
  },
  methods: {
    newMember() {
      this.$router.push({ name: 'org.members.new' });
    },
  },
});
</script>

<template>
  <div class="row">
    <div class="col-12">
      <q-table
        title="Members"
        dense
        :rows="members.content"
        :columns="columns"
        row-key="name"
        @request="fetchPageMembers"
        v-model:pagination="pagination"
      >
        <template v-slot:top>
          <div class="row full-width justify-between">
            <div class="text-h6">Members</div>
            <q-btn
              color="primary"
              icon="add"
              label="New member"
              @click="newMember"
            />
          </div>
        </template>
        <template v-slot:body="props">
          <q-tr :props="props">
            <q-td key="name" :props="props">
              {{
                props.row.person?.firstName + ' ' + props.row.person?.lastName
              }}
            </q-td>
            <q-td key="position" :props="props">
              {{ props.row.position.name }}
            </q-td>
            <q-td key="started" :props="props">
              {{ props.row.started }}
            </q-td>
            <q-td key="ended" :props="props">
              {{ props.row.ended }}
            </q-td>
            <q-td key="action" :props="props">
              <q-btn
                round
                icon="edit"
                color="primary"
                :to="'/org/members/' + props.row._id + '/edit'"
              ></q-btn>
            </q-td>
          </q-tr>
        </template>
      </q-table>
    </div>
  </div>
</template>

<style lang="sass" scoped></style>
