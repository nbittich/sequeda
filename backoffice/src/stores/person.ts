import { defineStore } from 'pinia';
import { api } from 'boot/axios';
import { ref } from 'vue';
export interface PersonState {
  _id?: string,
  userId?: string,
  firstName?: string,
  middleName?: string,
  lastName?: string,
  dateOfBirth?: Date,
  creationDate?: Date,
  updatedDate?: Date,
  nickname?: string,
  gender?: string,
  academicTitle?: string,
  contactDetail?: ContactDetail
}

export interface ContactDetail {
  emailAddress1?: string,
  elailAddress2?: string,
  phoneNumber1?: string,
  phoneNumber2?: string
  website?: string
  address?: Address
}

export interface Address {
  street?: string,
  number?: string,
  boxNumber?: string,
  postCode?: string
  municipality?: string,
  province?: string,
  country?: string,
}

export const usePersonStore = defineStore('person', {
  state: () => ({
    current: ref(null as unknown as PersonState)
  }),

  getters: {
    // doubleCount (state) {
    //   return state.counter * 2;
    // }
  },

  actions: {
    async fetchCurrent() {
      const data = await api.get<PersonState>('/person/current');
      this.current = data.data;
    },
  },
});
export default usePersonStore;
