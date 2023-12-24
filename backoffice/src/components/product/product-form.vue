<script lang="ts">
import { Product, productUnitTypes } from 'src/models/product';
import useProductStore from 'src/stores/product';
import { computed, defineComponent, ref } from 'vue';

const productStore = useProductStore();
export default defineComponent({
  name: 'ProductForm',
  props: {
    title: {
      type: String,
      default: () => '',
    },
    productModel: {
      type: Object,
      default: () => ({}) as Product,
    },
  },
  emits: ['update:productModel'],
  async setup(props, context) {
    const productComputed = computed({
      get: () => props.productModel,
      set: (value) => context.emit('update:productModel', value),
    });
    const product = ref(productComputed);
    let tagOptions = ref([] as string[]);
    let productTagsUpdated = ref(0);
    let unitUpdated = ref(0);

    return {
      product,
      unitUpdated,
      productTagsUpdated,
      productUnitTypes,
      tagOptions,
      async filterTags(val: string, update: (arg0: () => void) => void) {
        let tags: string[] = [val.trim()];
        if (val.trim().length) {
          tags = await productStore.searchTags(val.trim());
        }
        update(() => {
          tagOptions.value = tags;
        });
      },
      removeTag({ index }: { index: number; value: string }) {
        product.value.tags.splice(index, 1);
        productTagsUpdated.value += 1;
      },
    };
  },
  methods: {},
  components: {},
});
</script>
<template>
  <q-card>
    <q-card-section>
      <div class="row justify-between">
        <div class="text-h6">{{ title }}</div>
      </div>
    </q-card-section>

    <q-card-section>
      <div class="row q-mb-xs-none q-mb-md-xs">
        <div class="col-12">
          <q-input
            dense
            outlined
            class="q-mr-md-xs"
            label="Name"
            v-model="product.name"
          >
          </q-input>
        </div>
      </div>
      <div class="row q-mb-xs-none q-mb-md-xs">
        <div class="col-12">
          <q-input
            dense
            outlined
            type="textarea"
            class="q-mr-md-xs"
            label="description"
            v-model="product.description"
          >
          </q-input>
        </div>
      </div>
      <div class="row q-mb-xs-none q-mb-md-xs">
        <div class="col-6">
          <q-input
            dense
            outlined
            type="number"
            class="q-mr-md-xs"
            label="Price Per Unit"
            v-model.number="product.pricePerUnit"
          >
          </q-input>
        </div>
        <div class="col-6">
          <q-select
            :key="unitUpdated"
            dense
            outlined
            label="Unit Type"
            :options="productUnitTypes"
            option-label="label"
            option-value="value"
            @update:model-value="unitUpdated += 1"
            emit-value
            map-options
            v-model="product.unitType"
          >
          </q-select>
        </div>
      </div>
      <div class="row q-mb-xs-none q-mb-md-xs">
        <div class="col-12">
          <q-select
            :key="productTagsUpdated"
            class="q-mr-md-xs"
            dense
            outlined
            :options="tagOptions"
            v-model="product.tags"
            multiple
            use-chips
            use-input
            input-debounce="0"
            new-value-mode="add-unique"
            @filter="filterTags"
            @remove="removeTag"
            label="Choose existing tags"
          >
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
