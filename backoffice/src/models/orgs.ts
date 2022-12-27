import { BankAccount } from './bank-account';
import { ContactDetail, SelectOption } from './contact-detail';

export interface Position {
  _id?: string;
  name: string;
  description?: string;
  level: string;
  creationDate?: Date;
  updatedDate?: Date;
}
export const LEVEL = ['EXECUTIVE', 'MANAGEMENT', 'OPERATIONAL'];

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
  managedBy?: string;
  started?: string;
  ended?: string;
  remarks?: Remark[];
}
export interface Remark {
  id?: string;
  addedDate?: Date;
  updatedDate?: string;
  message: string;
  addedByUserId?: string;
  updatedByUserId?: string;
}
