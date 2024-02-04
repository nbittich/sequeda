import { SelectOption } from './shared';

export interface RenderRequest {
  templateId: string;
  context: Record<string, unknown>;
  fileName: string;
  templateContext: string;
}
export interface Template {
  _id?: string;
  title: string;
  description?: string;
  templateType: string;
  templateContext: string;
  creationDate?: Date;
  updatedDate?: Date;
  fileId?: string;
}
export const contexts: SelectOption[] = [
  {
    label: 'Invoice',
    value: 'INVOICE',
  },
];
export const types: SelectOption[] = [
  {
    label: 'Html',
    value: 'HTML',
  },
];
