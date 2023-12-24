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
    path: '/admin',
    component: () => import('layouts/MainLayout.vue'),
    children: [
      {
        path: '',
        component: () => import('pages/admin/AdminPage.vue'),
        children: [
          { path: 'logs', component: () => import('pages/admin/AuditLog.vue') },
        ],
      },
    ],
  },
  {
    path: '/product',
    component: () => import('layouts/MainLayout.vue'),
    children: [
      {
        path: '',
        name: 'products.root',
        component: () => import('pages/product/ProductPage.vue'),
        children: [
          {
            path: '/new',
            name: 'products.new',
            component: () => import('pages/product/NewProductPage.vue'),
          },
          {
            name: 'products.edit',
            path: ':id/edit',
            component: () => import('pages/product/EditProductPage.vue'),
          },
        ],
      },
    ],
  },

  {
    path: '/org',
    component: () => import('layouts/MainLayout.vue'),
    children: [
      {
        path: '',
        name: 'org.root',
        component: () => import('pages/organization/PersonalOrgPage.vue'),
        children: [
          {
            path: 'positions',
            children: [
              {
                name: 'org.positions.new',
                path: 'new',
                component: () =>
                  import('pages/organization/positions/NewOrgPositionPage.vue'),
              },
              {
                name: 'org.positions.edit',
                path: ':id/edit',
                component: () =>
                  import(
                    'pages/organization/positions/EditOrgPositionPage.vue'
                  ),
              },
              {
                name: 'org.positions.root',
                path: '',

                component: () =>
                  import('pages/organization/positions/OrgPositionPage.vue'),
              },
            ],
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
                path: ':id/edit',
                component: () =>
                  import('pages/organization/members/EditOrgMemberPage.vue'),
              },
              {
                path: '',
                name: 'org.members.root',
                component: () =>
                  import('pages/organization/members/OrgMembersPage.vue'),
              },
            ],
          },
          {
            path: 'customers',
            children: [
              {
                name: 'org.customers.new',
                path: 'new',
                component: () =>
                  import('pages/organization/customers/NewOrgCustomerPage.vue'),
              },
              {
                name: 'org.customers.edit',
                path: ':id/edit',
                component: () =>
                  import(
                    'pages/organization/customers/EditOrgCustomerPage.vue'
                  ),
              },
              {
                name: 'org.customers.root',
                path: '',
                component: () =>
                  import('pages/organization/customers/OrgCustomersPage.vue'),
              },
            ],
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
