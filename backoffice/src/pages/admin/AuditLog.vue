<script lang="ts">
import { defineComponent, ref } from 'vue';
import useAuditLogStore from 'src/stores/auditlog';
import { Pageable, PaginationProp } from 'src/models/pagination';
import { QTableColumn } from 'quasar';
const auditLogStore = useAuditLogStore();

const columns: QTableColumn[] = [
  {
    name: 'receivedDate',
    align: 'left',
    label: 'Date',
    field: 'receivedDate',
    sortable: false,
  },
  {
    name: 'message',
    label: 'Message',
    align: 'left',
    field: 'message',
    sortable: false,
  },
];

export default defineComponent({
  name: 'AuditLog',
  components: {},
  computed: {},
  async setup() {
    const pageRequest = ref({ limit: 10, page: 0 } as Pageable);
    const logs = ref(await auditLogStore.fetchAuditLogs(pageRequest.value));
    const pagination = ref({
      page: pageRequest.value.page - 1,
      rowsPerPage: pageRequest.value.limit,
      rowsNumber: logs.value.totalElements,
    });
    const fetchPageLogs = async (props: PaginationProp) => {
      const { page, rowsPerPage } = props.pagination;
      pageRequest.value.page = page - 1;
      pageRequest.value.limit = rowsPerPage;
      logs.value = await auditLogStore.fetchAuditLogs(pageRequest.value);
      pagination.value = {
        page: logs.value.currentPage + 1,
        rowsPerPage: pageRequest.value.limit,
        rowsNumber: logs.value.totalElements,
      };
    };

    return { logs, pageRequest, columns, fetchPageLogs, pagination };
  },
  methods: {},
});
</script>

<template>
  <div class="row">
    <div class="col-12">
      <q-table
        title="Audit Logs"
        dense
        :rows="logs.content"
        :columns="columns"
        row-key="id"
        @request="fetchPageLogs"
        v-model:pagination="pagination"
      />
    </div>
  </div>
</template>

<style lang="sass" scoped></style>
