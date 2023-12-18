import { defineStore } from 'pinia';
import { api } from 'boot/axios';
import { OrgCustomer, OrgCustomerDetail } from 'src/models/orgs';
import { Page, Pageable, toQueryString } from 'src/models/pagination';

const useCustomerStore = defineStore('customerStore', {
  state: () => ({}),

  getters: {},

  actions: {
    async fetchCustomers(
      orgId: string,
      pageable: Pageable = { page: 0, limit: 20 },
    ): Promise<Page<OrgCustomerDetail>> {
      const response = await api.get<Page<OrgCustomerDetail>>(
        `/orgs/customers/find-by-org/${orgId}?${toQueryString(pageable)}`,
      );
      return response.data;
    },
    async findOne(customerId: string): Promise<OrgCustomer> {
      const response = await api.get<OrgCustomer>(
        `/orgs/customers/find-one/${customerId}`,
      );
      return response.data;
    },
    async update(customer: OrgCustomer) {
      const response = await api.post<OrgCustomer>('/orgs/customers', customer);
      return response.data;
    },
  },
});

export default useCustomerStore;
