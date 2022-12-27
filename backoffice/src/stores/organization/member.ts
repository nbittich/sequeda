import { defineStore } from 'pinia';
import { api } from 'boot/axios';
import { Organization, OrgMember } from 'src/models/orgs';
import { Page, Pageable, toQueryString } from 'src/models/pagination';

const useMemberStore = defineStore('memberStore', {
  state: () => ({}),

  getters: {},

  actions: {
    async fetchMembers(
      orgId: string,
      pageable: Pageable = { page: 0, limit: 20 }
    ): Promise<Page<OrgMember>> {
      const response = await api.get<Page<OrgMember>>(
        `/orgs/members/find-by-org/${orgId}?${toQueryString(pageable)}`
      );
      return response.data;
    },
    async update(member: OrgMember) {
      const response = await api.post<OrgMember>('/orgs/members', member);
      return response.data;
    },
  },
});

export default useMemberStore;
