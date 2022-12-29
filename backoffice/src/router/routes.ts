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
    path: '/org/positions',
    component: () => import('layouts/MainLayout.vue'),
    children: [
      {
        path: '',
        component: () => import('pages/organization/OrgPositionPage.vue'),
      },
    ],
  },
  {
    path: '/org/current',
    component: () => import('layouts/MainLayout.vue'),
    children: [
      {
        path: '',
        component: () => import('pages/organization/PersonalOrgPage.vue'),
      },
    ],
  },
  {
    path: '/org/members',
    component: () => import('layouts/MainLayout.vue'),
    children: [
      {
        path: '',
        component: () =>
          import('pages/organization/members/OrgMembersPage.vue'),
      },
    ],
  },
  {
    path: '/org/members/new',
    component: () => import('layouts/MainLayout.vue'),
    children: [
      {
        path: '',
        component: () =>
          import('pages/organization/members/NewOrgMemberPage.vue'),
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
