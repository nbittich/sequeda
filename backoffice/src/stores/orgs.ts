import { defineStore } from 'pinia';
import { api } from 'boot/axios';
import { Organization } from 'src/models/orgs';

const useOrgsStore = defineStore('orgs', {
  state: () => ({
    current: null as unknown as Organization,
  }),

  getters: {},

  actions: {
    async fetchCurrent() {
      const response = await api.get<Organization>('/orgs/current');
      this.current = response.data;
    },
    async update(org: Organization) {
      const response = await api.post<Organization>('/orgs', org);
      return response.data;
    },
  },
});

export default useOrgsStore;
