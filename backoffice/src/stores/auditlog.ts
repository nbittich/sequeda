import { defineStore } from 'pinia';
import { api } from 'boot/axios';
import { Page, Pageable } from './pagination';
export interface Country {
  code: string,
  label: string
}

export interface AuditLog {
  _id: string,
  receivedDate: string,
  message: string,
}

const useAuditLogStore = defineStore('auditLog', {
  state: () => ({
    logs: null as unknown as Page<AuditLog>
  }),

  getters: {

  },

  actions: {
    async fetchAuditLogs(pageable: Pageable = {page: 0, limit:20, } as Pageable) {
      const response = await api.get<Page<AuditLog>>(`audit-log/find-all?page=${pageable.page}&limit=${pageable.limit}${pageable.sort ? '&sort='+ JSON.stringify(pageable.sort):''}`);
      this.logs = response.data || [];
      return this.logs;
    },

  },
});
export default useAuditLogStore;
