import { defineStore } from 'pinia';
import { api } from 'boot/axios';
import { Page, Pageable, toQueryString } from '../models/pagination';
import { AuditLog } from 'src/models/auditlog';

const useAuditLogStore = defineStore('auditLog', {
  state: () => ({
    logs: null as unknown as Page<AuditLog>,
  }),

  getters: {},

  actions: {
    async fetchAuditLogs(pageable: Pageable = { page: 0, limit: 20 }) {
      const response = await api.get<Page<AuditLog>>(
        `audit-log/find-all?${toQueryString(pageable)}`
      );
      this.logs = response.data || {};
      return this.logs;
    },
  },
});
export default useAuditLogStore;
