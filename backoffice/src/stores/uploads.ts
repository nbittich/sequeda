import { defineStore } from 'pinia';
import { api } from 'boot/axios';
import { FileUpload } from 'src/models/file-upload';

const useUploadStore = defineStore('upload', {
  state: () => ({
    uploads: [] as FileUpload[],
  }),

  getters: {},

  actions: {
    async uploadFile(
      file: File,
      id?: string,
      correlationId?: string,
      publicResource = false,
    ): Promise<FileUpload> {
      const multipart = new FormData();
      multipart.append('file', file);
      const urlParams = new URLSearchParams();
      if (id?.length) {
        urlParams.append('id', id);
      }
      if (correlationId?.length) {
        urlParams.append('correlation_id', correlationId);
      }
      if (publicResource) {
        urlParams.append('public', 'true');
      }

      const response = await api.post<FileUpload>(
        `/uploads/upload?${urlParams.toString()}`,
        multipart,
      );
      return response.data;
    },
    getDownloadUrl(id?: string) {
      return `/api/uploads/download?id=${id}`;
    },
    async getMetadata(id: string): Promise<FileUpload> {
      const resp = await api.get<FileUpload>(`/uploads/metadata?id=${id}`);
      return resp.data;
    },
  },
});
export default useUploadStore;
