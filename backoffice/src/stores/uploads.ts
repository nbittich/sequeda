import { defineStore } from 'pinia';
import { api } from 'boot/axios';

export interface FileUpload {
  _id: string;
  creationDate?: Date;
  updatedDate?: Date;
  thumbnailId?: string;
  contentType?: string;
  originalFilename?: string;
  extension?: string;
  size?: number;
  publicResource?: number;
  correlationId?: number;
}

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
      publicResource = false
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
        multipart
      );
      return response.data;
    },
    getDownloadUrl(id: string) {
      return `/api/uploads/download/${id}`;
    },
  },
});
export default useUploadStore;
