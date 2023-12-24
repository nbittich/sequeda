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

    return { columns, products };
  },
  methods: {
    newProduct() {
      this.$router.push({ name: 'org.products.new' });
    },
  },
});
</script>

<template>
  <q-table dense :rows="products" :columns="columns" row-key="_id">
    <template v-slot:top>
      <div class="row full-width justify-between">
        <div class="text-h6">Products</div>
        <q-btn color="primary" icon="add" label="New position" @click="newProduct" />
      </div>
    </template>
  </q-table>
</template>

<style lang="sass" scoped></style>
