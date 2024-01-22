import { SelectOption } from './shared';

export interface Template {
  _id?: string;
  title: string;
  description?: string;
  templateType: string;
  templateContext: string;
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
