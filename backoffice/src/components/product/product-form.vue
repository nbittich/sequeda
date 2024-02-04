<script lang="ts">
import { Product } from 'src/models/product';
import { unitTypes } from 'src/models/shared';
import useProductStore from 'src/stores/product';
import { computed, defineComponent, ref } from 'vue';
import ImageUpload from 'src/components/shared/image-upload.vue';
const productStore = useProductStore();
export default defineComponent({
  name: 'ProductForm',
  props: {
    title: {
      type: String,
      default: () => '',
    },

    imageKey: {
      type: Number,
      default: () => ref(0),
    },
    mainPicture: {
      type: Object,
      default: () => ({}) as File,
    },
    productModel: {
      type: Object,
      default: () => ({}) as Product,
    },
  },
  emits: ['update:productModel', 'update:mainPicture'],
  async setup(props, context) {
    const productComputed = computed({
      get: () => props.productModel,
      set: (value) => context.emit('update:productModel', value),
    });
    const product = ref(productComputed);
    let tagOptions = ref([] as string[]);
    let productTagsUpdated = ref(0);
    let unitUpdated = ref(0);
    const mainPictureFile = computed({
      get: () => props.mainPicture,
      set: (value) => context.emit('update:mainPicture', value),
    });

    return {
      product,
      unitUpdated,
      mainPictureFile,
      productTagsUpdated,
      unitTypes,
      tagOptions,
      async filterTags(val: string, update: (arg0: () => void) => void) {
        if (val.trim().length) {
          let tags: string[] = [val.trim()];

          tags = await productStore.searchTags(val.trim());
          update(() => {
            tagOptions.value = tags;
          });
        }
      },
      removeTag({ index }: { index: number; value: string }) {
        product.value.tags.splice(index, 1);
        productTagsUpdated.value += 1;
      },
    };
  },
  methods: {},
  components: { ImageUpload },
});
</script>
<template>
  <q-card>
    <q-card-section>
      <div class="row justify-between">
        <div class="text-h6">{{ title }}</div>
      </div>
    </q-card-section>
    <q-card-section class="row justify-center">
      <div class="col">
        <div class="text-h6 q-ml-xs-sm">Picture Product</div>
        <ImageUpload :key="imageKey" v-model="mainPictureFile" :picture-id="product.mainPictureId" />
      </div>
    </q-card-section>
    <q-separator />

    <q-card-section>
      <div class="row q-mb-xs-none q-mb-md-xs">
        <div class="col-6">
          <q-input dense outlined class="q-mr-md-xs" label="Label" v-model="product.label">
          </q-input>
        </div>
        <div class="col-6">
          <q-input dense outlined class="q-mr-md-xs" label="Name" v-model="product.name">
          </q-input>
        </div>
      </div>
      <div class="row q-mb-xs-none q-mb-md-xs">
        <div class="col-12">
          <q-input dense outlined type="textarea" class="q-mr-md-xs" label="description" v-model="product.description">
          </q-input>
        </div>
      </div>
      <div class="row q-mb-xs-none q-mb-md-xs">
        <div class="col-3">
          <q-input dense outlined type="number" class="q-mr-md-xs" label="Price Per Unit"
            v-model.number="product.pricePerUnit">
          </q-input>
        </div>
        <div class="col-3">
          <q-input dense outlined type="number" class="q-mr-md-xs" min="0" max="100" label="VAT"
            v-model.number="product.vat">
          </q-input>
        </div>
        <div class="col-6">
          <q-select :key="unitUpdated" dense outlined label="Unit Type" :options="unitTypes" option-label="label"
            option-value="value" @update:model-value="unitUpdated += 1" emit-value map-options v-model="product.unitType">
          </q-select>
        </div>
      </div>
      <div class="row q-mb-xs-none q-mb-md-xs">
        <div class="col-12">
          <q-select :key="productTagsUpdated" class="q-mr-md-xs" dense outlined :options="tagOptions"
            v-model="product.tags" multiple use-chips use-input emit-value input-debounce="0" new-value-mode="add-unique"
            @filter="filterTags" @remove="removeTag" label="Choose existing tags">
            <template v-slot:no-option>
              <q-item>
                <q-item-section class="text-grey"> No results </q-item-section>
              </q-item>
            </template>
          </q-select>
        </div>
      </div>
    </q-card-section>
  </q-card>
</template>
