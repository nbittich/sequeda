<script lang="ts">
import { defineComponent, ref } from 'vue';
import { QTableColumn } from 'quasar';
import useProductStore from 'src/stores/product';
const productStore = useProductStore();
const columns: QTableColumn[] = [
  {
    name: 'name',
    align: 'left',
    label: 'Name',
    field: 'name',
    sortable: true,
  },
  {
    name: 'price',
    align: 'left',
    label: 'Price',
    field: 'pricePerUnit',
    sortable: true,
  },
  {
    name: 'description',
    align: 'left',
    label: 'Description',
    field: 'description',
    sortable: false,
  },
  {
    name: 'action',
    align: 'left',
    label: 'Action',
    field: 'action',
    sortable: false,
  },
];

export default defineComponent({
  name: 'ProductPage',
  components: {},
  computed: {},
  async setup() {
    const products = ref(await productStore.findAll());

    return {
      columns,
      products,
      tab: ref('general'), // in case adding more tabs, see PersonalOrgPage.vue for an example
    };
  },
  methods: {
    newProduct() {
      this.$router.push({ name: 'products.new' });
    },
  },
});
</script>

<template>
  <q-tabs v-model="tab" class="text-teal" inline-label outside-arrows mobile-arrows>
    <q-route-tab to="/product" name="general" icon="perm_identity" label="General" />
  </q-tabs>
  <q-separator />

  <q-tab-panels v-model="tab" v-if="products" animated>
    <q-tab-panel name="general" v-if="$route.name === 'products.root'">
      <q-table dense :rows="products" :columns="columns" row-key="_id">
        <template v-slot:top>
          <div class="row full-width justify-between">
            <div class="text-h6">Products</div>
            <q-btn color="primary" icon="add" label="New product" @click="newProduct" />
          </div>
        </template>
        <template v-slot:body="props">
          <q-tr :props="props">
            <q-td key="name" :props="props">
              {{ props.row.name }}
            </q-td>
            <q-td key="price" :props="props">
              {{ props.row.pricePerUnit }}
            </q-td>
            <q-td key="description" :props="props">
              {{ props.row.description }}
            </q-td>
            <q-td key="action" :props="props">
              <q-btn round icon="edit" color="primary" :to="'/product/' + props.row._id + '/edit'"></q-btn>
            </q-td>
          </q-tr>
        </template>
      </q-table>
    </q-tab-panel>

    <q-tab-panel name="general" v-if="$route.name !== 'products.root'">
      <router-view :key="($route.params.id as string) || $route.query.t?.toString()" />
    </q-tab-panel>
  </q-tab-panels>
</template>

<style lang="sass" scoped></style>
