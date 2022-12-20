import { defineStore } from 'pinia';
import { api } from 'boot/axios';

export interface Position {
  _id?: string,
  name: string,
  description?: string,
  level: string,
  creationDate?: Date,
  updatedDate?: Date,
}

export const LEVEL = ['EXECUTIVE, MANAGEMENT, OPERATIONAL'];

const useOrgPositionStore = defineStore('org-position', {
  state: () => ({
    positions: null as unknown as Position[]
  }),

  getters: {

  },

  actions: {
    async fetchPositions() {
      const response = await api.get<Position[]>(`org-position/find-all`);
      this.positions = response.data || [];
      return this.positions;
    },

  },
});
export default useOrgPositionStore;
