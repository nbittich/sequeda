<script lang="ts">
import { QTableColumn } from 'quasar';
import {
  Organization,
  OrgCustomerDetail,
  representedByIsOrg,
} from 'src/models/orgs';
import { Page, Pageable, PaginationProp } from 'src/models/pagination';
import { Person } from 'src/models/person';
import useCustomerStore from 'src/stores/organization/customer';
import useOrgsStore from 'src/stores/organization/orgs';
import usePersonStore from 'src/stores/person';
import { defineComponent, ref } from 'vue';

const customerStore = useCustomerStore();
const personStore = usePersonStore();
const orgStore = useOrgsStore();

const currentOrg = await orgStore.fetchCurrent();
const columns: QTableColumn[] = [
  {
    name: 'name',
    align: 'left',
    label: 'Name',
    sortable: false,
    field: (row) => row.name,
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
  name: 'OrgCustomers',
  components: {},
  computed: {},
  async setup() {
    const pageRequest = ref({ limit: 10, page: 0 } as Pageable);
    const customers = ref(null as unknown as Page<OrgCustomerDetail>);
    const pagination = ref({ page: 1, rowsPerPage: 10 } as {
      page: number;
      rowsPerPage: number;
      rowsNumber: number;
    });
    const fetchPageCustomers = async (props: PaginationProp) => {
      const { page, rowsPerPage } = props.pagination;
      pageRequest.value.page = (page || 1) - 1;
      pageRequest.value.limit = rowsPerPage || pageRequest.value.limit;
      customers.value = await customerStore.fetchCustomers(
        currentOrg._id as string,
        pageRequest.value,
      );
      if (customers.value?.content?.length) {
        const personIds = customers.value?.content
          .filter(
            (c: OrgCustomerDetail) =>
              typeof c.representedById == 'string' &&
              c.customerType === 'PERSON',
          )
          .map((m) => m.representedById as string);
        const persons = await personStore.findByIds(personIds);

        const orgIds = customers.value?.content
          .filter(
            (c: OrgCustomerDetail) =>
              typeof c.representedById == 'string' &&
              c.customerType === 'ORGANIZATION',
          )
          .map((m) => m.representedById as string);
        const organizations = await orgStore.findByIds(orgIds);

        for (const customer of customers.value?.content) {
          switch (customer.customerType) {
            case 'PERSON':
              customer.representedBy = persons.find(
                (p) => p._id === customer.representedById,
              );
              break;
            case 'ORGANIZATION':
              customer.representedBy = organizations.find(
                (o) => o._id === customer.representedById,
              );
              break;
            default:
              throw Error('Could not determine customer type!');
          }
        }
        pagination.value = {
          page: customers.value.currentPage + 1,
          rowsPerPage: pageRequest.value.limit,
          rowsNumber: customers.value.totalElements,
        };
      }
    };
    const getName = (representedBy: Person | Organization) => {
      if (representedByIsOrg(representedBy)) {
        return representedBy.name;
      } else {
        return representedBy.firstName + ' ' + representedBy.lastName;
      }
    };
    await fetchPageCustomers({ pagination: pagination.value });
    return {
      customers,
      pageRequest,
      columns,
      getName,
      fetchPageCustomers,
      pagination,
    };
  },
  methods: {
    newCustomer() {
      this.$router.push({ name: 'org.customers.new' });
    },
  },
});
</script>

<template>
  <div class="row">
    <div class="col-12">
      <q-table
        title="customers"
        dense
        :rows="customers.content"
        :columns="columns"
        row-key="name"
        @request="fetchPageCustomers"
        v-model:pagination="pagination"
      >
        <template v-slot:top>
          <div class="row full-width justify-between">
            <div class="text-h6">Customers</div>
            <q-btn
              color="primary"
              icon="add"
              label="New customer"
              @click="newCustomer"
            />
          </div>
        </template>
        <template v-slot:body="props">
          <q-tr :props="props">
            <q-td key="name" :props="props">
              {{ getName(props.row.representedBy) }}
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
                :to="'/org/customers/' + props.row._id + '/edit'"
              ></q-btn>
            </q-td>
          </q-tr>
        </template>
      </q-table>
    </div>
  </div>
</template>

<style lang="sass" scoped></style>
