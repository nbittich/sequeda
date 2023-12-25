import { BankAccount } from './bank-account';
import { ContactDetail } from './contact-detail';
import { SelectOption } from './shared';
import { Person } from './person';

export interface Position {
  _id?: string;
  name: string;
  description?: string;
  level?: string;
  creationDate?: Date;
  updatedDate?: Date;
}
export const positionLevel: SelectOption[] = [
  { label: 'Executive', value: 'EXECUTIVE' },
  { label: 'Management', value: 'MANAGEMENT' },
  { label: 'Operational', value: 'OPERATIONAL' },
  { label: 'Junior', value: 'JUNIOR' },
  { label: 'Medior', value: 'MEDIOR' },
  { label: 'Senior', value: 'SENIOR' },
];

export const orgStatuses: SelectOption[] = [
  {
    label: 'Active',
    value: 'ACTIVE',
  },
  {
    label: 'Proposal to strike off',
    value: 'PROPOSAL_TO_STRIKE_OFF',
  },
  {
    label: 'Dissolved',
    value: 'DISSOLVED',
  },
  {
    label: 'Liquidation',
    value: 'LIQUIDATION',
  },
];

export const customerTypes: SelectOption[] = [
  {
    label: 'Person',
    value: 'PERSON',
  },
  {
    label: 'Organization',
    value: 'ORGANIZATION',
  },
];

export interface Organization {
  _id?: string;
  creationDate?: Date;
  updatedDate?: Date;
  name: string;
  description?: string;
  vatNumber?: string;
  logoId?: string;
  primaryContact?: ContactDetail;
  otherContacts?: ContactDetail[];
  primaryBankAccount?: BankAccount;
  otherBankAccounts?: BankAccount[];
  foundedDate?: string;
  closedDate?: string;
  status?: string;
}
export interface OrgMember {
  _id?: string;
  creationDate?: Date;
  updatedDate?: Date;
  orgId?: string;
  personId?: string;
  positionId?: string;
  responsibleOf?: string[];
  managedBy: string[];
  started?: string;
  ended?: string;
  remarks?: Remark[];
}
export interface OrgMemberDetail extends OrgMember {
  position?: Position;
  person?: Person;
}

export interface OrgCustomer {
  _id?: string;
  creationDate?: Date;
  updatedDate?: Date;
  orgId?: string;
  representedById?: string;
  customerType: string;
  recurringProductIds?: string[];
  started?: string;
  ended?: string;
  documentIds?: string[];
  communications?: Communication[];
}

export type RepresentedBy = Person | Organization;

export function representedByIsOrg(v: RepresentedBy): v is Organization {
  return 'vatNumber' in v;
}

export interface OrgCustomerDetail extends OrgCustomer {
  representedBy?: RepresentedBy;
}

export type Communication = Remark;

export interface Remark {
  id?: string;
  addedDate?: Date;
  updatedDate?: string;
  message: string;
  addedByUserId?: string;
  updatedByUserId?: string;
}
