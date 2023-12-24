import { defineStore } from 'pinia';
import { api } from 'boot/axios';
import { Product } from 'src/models/product';

const useProductStore = defineStore('product', {
  state: () => ({}),

  getters: {},

  actions: {
    async findOne(productId: string) {
      const response = await api.get<Product>(`/product/find-one/${productId}`);
      return response.data;
    },
    async findAll() {
      const response = await api.get<Product[]>('/product/find-all');
      return response.data;
    },
    async findByIds(ids: string[]): Promise<Product[]> {
      const response = await api.post<Product[]>('/product/find-by-ids', ids);
      return response.data;
    },
    async update(product: Product) {
      const response = await api.post<Product>('/product', product);
      return response.data;
    },
    async searchTags(tag: string): Promise<string[]> {
      const response = await api.get<string[]>(
        `/product/tag/search?tag=${tag}`,
      );
      return response.data;
    },
  },
});
export default useProductStore;
