import { BankAccount } from './bank-account';
import { ContactDetail } from './contact-detail';

export interface Invoice {
  _id?: string;
  templateId?: string;
  dateOfInvoice: string;
  creationDate?: Date;
  updatedDate?: Date;
  number?: string;
  reference?: string;
  notes?: string[];
  locked: boolean;
  customer: Customer;
  invoicer: Invoicer;
  items: InvoiceItem[];
}

export interface Invoicer {
  logoId?: string;
  invoicerId?: string;
  isOrg: boolean;
  invoicerName: string;
  vatNumber?: string;
  contactDetail: ContactDetail;
  bankAccounts: BankAccount[];
}
export interface Customer {
  customerId?: string;
  customerName: string;
  vatNumber?: string;
  contactDetail: ContactDetail;
}

export interface InvoiceItem {
  name: string;
  description?: string;
  qty: number;
  vat: number; // e.g 21 for 21%
  pricePerUnit: number;
  unitType: string;
}
