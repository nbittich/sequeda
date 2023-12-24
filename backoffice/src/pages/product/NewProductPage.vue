<script lang="ts">
//import useUploadStore from 'src/stores/uploads';
import useProductStore from 'src/stores/product';
import { defineComponent, ref } from 'vue';
import ProductForm from 'src/components/product/product-form.vue';
import { Product } from 'src/models/product';
//const uploadStore = useUploadStore();

const productStore = useProductStore();
export default defineComponent({
  name: 'NewProductPage',
  components: { ProductForm },
  computed: {},
  async setup() {
    const product = ref({ pricePerUnit: 0, tags: [] as string[] } as Product);
    const pictureFile = ref(null as unknown as File);
    return {
      pictureFile,
      product,
    };
  },
  methods: {
    async update() {
      await productStore.update(this.product);
      this.$router.push({ name: 'products.root' });
    },
    async reset() {
      this.$router.push({ name: 'products.root' });
    },
  },
});
</script>

<template>
  <div class="row">
    <div class="col-12">
      <q-card>
        <!-- <PersonForm v-if="customerType == 'PERSON'" :title="'Person'" v-model:person-model="representedByPerson" -->
        <!--   v-model:profile-picture="pictureFile" /> -->
        <!---->
        <ProductForm v-model:product-model="product" :title="'New Product'" />

        <q-separator />
        <q-card-actions>
          <q-btn color="primary" @click="update">Save</q-btn>
          <q-btn color="deep-orange" @click="reset">Cancel</q-btn>
        </q-card-actions>
      </q-card>
    </div>
  </div>
</template>

<style lang="sass" scoped></style>
