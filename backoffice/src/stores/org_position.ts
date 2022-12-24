import { defineStore } from 'pinia';
import { api } from 'boot/axios';
import { Position } from 'src/models/orgs';

const useOrgPositionStore = defineStore('org-position', {
  state: () => ({
    positions: null as unknown as Position[],
  }),

  getters: {},

  actions: {
    async fetchPositions() {
      const response = await api.get<Position[]>('orgs/position/find-all');
      this.positions = response.data || [];
      return this.positions;
    },
  },
});

export default useOrgPositionStore;
