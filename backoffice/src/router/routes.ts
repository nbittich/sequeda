import { RouteRecordRaw } from 'vue-router';

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    component: () => import('layouts/MainLayout.vue'),
    children: [{ path: '', component: () => import('pages/IndexPage.vue') }],
  },
  {
    path: '/personal-info',
    component: () => import('layouts/MainLayout.vue'),
    children: [
      {
        path: '',
        component: () => import('pages/PersonalInformationPage.vue'),
      },
    ],
  },
  {
    path: '/audit',
    component: () => import('layouts/MainLayout.vue'),
    children: [{ path: '', component: () => import('pages/AuditLog.vue') }],
  },

  {
    path: '/org',
    component: () => import('layouts/MainLayout.vue'),
    children: [
      {
        path: 'current',
        component: () => import('pages/organization/PersonalOrgPage.vue'),
      },
      {
        path: 'positions',
        component: () => import('pages/organization/OrgPositionPage.vue'),
      },
      {
        path: 'members',
        children: [
          {
            name: 'org.members.new',
            path: 'new',
            component: () =>
              import('pages/organization/members/NewOrgMemberPage.vue'),
          },
          {
            name: 'org.members.edit',
            path: 'edit/:id',
            component: () =>
              import('pages/organization/members/EditOrgMemberPage.vue'),
          },
          {
            name: 'org.members.root',
            path: '',
            component: () =>
              import('pages/organization/members/OrgMembersPage.vue'),
          },
        ],
      },
    ],
  },

  // Always leave this as last one,
  // but you can also remove it
  {
    path: '/:catchAll(.*)*',
    component: () => import('pages/ErrorNotFound.vue'),
  },
];

export default routes;
