import { defineStore } from 'pinia';
import { api } from 'boot/axios';
export interface Person {
  _id?: string;
  userId?: string;
  firstName?: string;
  middleName?: string;
  lastName?: string;
  profilePictureId?: string;
  dateOfBirth?: string;
  creationDate?: Date;
  updatedDate?: Date;
  nickName?: string;
  gender?: string;
  maritalStatus?: string;
  academicTitle?: string;
  contactDetail: ContactDetail;
}

export interface ContactDetail {
  emailAddress1?: string;
  emailAddress2?: string;
  phoneNumber1?: string;
  phoneNumber2?: string;
  website?: string;
  address: Address;
}

export interface Address {
  street?: string;
  number?: string;
  boxNumber?: string;
  postCode?: string;
  municipality?: string;
  province?: string;
  country?: string;
}

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
    async update(person: Person) {
      const response = await api.post<Person>('/person', person);
      return response.data;
    },
  },
});
export default usePersonStore;
