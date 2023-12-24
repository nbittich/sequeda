<script lang="ts">
import useUploadStore from 'src/stores/uploads';
import useProductStore from 'src/stores/product';
import { defineComponent, ref } from 'vue';
import ProductForm from 'src/components/product/product-form.vue';
import { useRoute } from 'vue-router';
const uploadStore = useUploadStore();

const imageKey = ref(0);
const reload = () => {
  imageKey.value += 1;
};
const productStore = useProductStore();
export default defineComponent({
  name: 'EditProductPage',
  components: { ProductForm },
  computed: {},
  async setup() {
    const route = useRoute();
    const product = ref(await productStore.findOne(route.params.id as string));
    const pictureFile = ref(null as unknown as File);
    return {
      pictureFile,
      product,
      imageKey,
      reload,
    };
  },
  methods: {
    async update() {
      const product = await productStore.update(this.product);

      if (this.pictureFile) {
        const upload = await uploadStore.uploadFile(
          this.pictureFile,
          product.mainPictureId,
          product._id,
        );
        product.mainPictureId = upload._id;
        this.pictureFile = null as unknown as File;
        await productStore.update(product);
      }

      this.$router.push({
        name: 'products.root',
        query: { t: new Date().getTime() },
      });
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
        <ProductForm
          v-model:product-model="product"
          v-model:main-picture="pictureFile"
          :image-key="imageKey"
          :title="'Edit Product'"
        />

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
