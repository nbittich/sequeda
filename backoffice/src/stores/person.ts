import { defineStore } from 'pinia';
import { api } from 'boot/axios';
import { Person } from 'src/models/person';

const usePersonStore = defineStore('person', {
  state: () => ({
    current: null as unknown as Person,
  }),

  getters: {},

  actions: {
    async fetchCurrent() {
      const response = await api.get<Person>('/person/current');
      this.current = response.data;
    },
    async findByIds(ids: string[]): Promise<Person[]> {
      const response = await api.post<Person[]>('/person/find-by-ids', ids);
      return response.data;
    },
    async update(person: Person) {
      const response = await api.post<Person>('/person', person);
      return response.data;
    },
  },
});
export default usePersonStore;
