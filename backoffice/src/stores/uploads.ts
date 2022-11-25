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
     getDownloadUrl(id: string) {
      return api.get<FileUpload>(`/uploads/downloads/${id}`);
    },
  },
});
export default useUploadStore;
