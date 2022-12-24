import { defineStore } from 'pinia';
import { api } from 'boot/axios';
import { Country, PostalCode } from 'src/models/contact-detail';

const useGeoStore = defineStore('geo', {
  state: () => ({
    countries: [] as Country[],
  }),

  getters: {},

  actions: {
    async fetchCountries() {
      const response = await api.get<Country[]>('/geo/countries');
      this.countries = response.data;
    },
    async postCodesByQuery(country: Country, query: string) {
      const response = await api.get<PostalCode[]>(
        `/geo/municipality/by-query?country_code=${country.code}&query=${query}`
      );
      return response.data;
    },
  },
});
export default useGeoStore;
