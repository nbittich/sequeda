import { defineStore } from 'pinia';
import { api } from 'boot/axios';
import { RenderRequest, Template } from 'src/models/template';

const useTemplateStore = defineStore('template', {
  state: () => ({}),

  getters: {},

  actions: {
    async upsert(
      file: File,
      title: string,
      templateType: string,
      templateContext: string,
      id?: string,
      description?: string,
    ): Promise<Template> {
      const multipart = new FormData();
      multipart.append('file', file);
      const urlParams = new URLSearchParams();
      urlParams.append('title', title);
      urlParams.append('templateType', templateType);
      urlParams.append('templateContext', templateContext);
      if (id?.length) {
        urlParams.append('id', id);
      }
      if (description?.length) {
        urlParams.append('description', description);
      }

      const response = await api.post<Template>(
        `/template?${urlParams.toString()}`,
        multipart,
      );
      return response.data;
    },
    async findAll() {
      const response = await api.get<Template[]>('/template/find-all');
      return response.data;
    },
    async findOne(id: string) {
      const response = await api.get<Template>(`/template/find-one/${id}`);
      return response.data;
    },
    async findByContext(context: string) {
      const response = await api.get<Template[]>(
        '/template/find-by-context?context=' + context,
      );
      return response.data;
    },
    async findByIds(ids: string[]): Promise<Template[]> {
      const response = await api.post<Template[]>('/template/find-by-ids', ids);
      return response.data;
    },
    async render(req: RenderRequest) {
      const response = await api.post<Blob>('/template/render', req, {
        responseType: 'blob',
      });
      const href = URL.createObjectURL(response.data);

      const link = document.createElement('a');
      link.href = href;
      link.setAttribute(
        'download',
        `${req.fileName}_${crypto.randomUUID()}.pdf`,
      );
      document.body.appendChild(link);
      link.click();

      // clean up "a" element & remove ObjectURL
      document.body.removeChild(link);
      URL.revokeObjectURL(href);
    },
  },
});
export default useTemplateStore;
