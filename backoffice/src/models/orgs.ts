import { BankAccount } from './bank-account';
import { ContactDetail } from './contact-detail';

export interface Position {
  _id?: string;
  name: string;
  description?: string;
  level: string;
  creationDate?: Date;
  updatedDate?: Date;
}
export const LEVEL = ['EXECUTIVE', 'MANAGEMENT', 'OPERATIONAL'];
export const STATUS = [
  'ACTIVE',
  'PROPOSAL_TO_STRIKE_OFF',
  'DISSOLVED',
  'LIQUIDATION',
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
  bankAccounts?: BankAccount[];
  foundedDate?: string;
  closedDate?: string;
  status?: string;
}
