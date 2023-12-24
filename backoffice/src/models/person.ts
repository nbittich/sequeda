import { BankAccount } from './bank-account';
import { ContactDetail } from './contact-detail';
import { SelectOption } from './shared';
export const genders: SelectOption[] = [
  {
    label: 'Male',
    value: 'MALE',
  },
  {
    label: 'Female',
    value: 'FEMALE',
  },
  {
    label: 'Unknown',
    value: 'UNKNOWN',
  },
];

export const academicTitles: SelectOption[] = [
  {
    label: '-',
    value: null,
  },
  {
    label: 'Doctor',
    value: 'DR',
  },
  {
    label: 'Professor',
    value: 'PROFESSOR',
  },
];

export const maritalStatuses: SelectOption[] = [
  {
    label: '-',
    value: null,
  },
  {
    label: 'Single',
    value: 'SINGLE',
  },
  {
    label: 'Married',
    value: 'MARRIED',
  },
  {
    label: 'Divorced',
    value: 'DIVORCED',
  },
  {
    label: 'Separated',
    value: 'SEPARATED',
  },
  {
    label: 'Civil Partnership',
    value: 'CIVIL_PARTNERSHIP',
  },
  {
    label: 'Widowed',
    value: 'WIDOWED',
  },
];

export interface Person {
  _id?: string;
  userId?: string;
  firstName?: string;
  middleName?: string;
  lastName?: string;
  profilePictureId?: string;
  signatureId?: string;
  dateOfBirth?: string;
  creationDate?: Date;
  updatedDate?: Date;
  nickName?: string;
  gender?: string;
  maritalStatus?: string;
  academicTitle?: string;
  bankAccount?: BankAccount;
  contactDetail: ContactDetail;
}
