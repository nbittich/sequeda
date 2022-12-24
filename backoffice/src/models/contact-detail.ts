export type SelectOption = { label: string; value: string | null };

export interface ContactDetail {
  emailAddress1?: string;
  emailAddress2?: string;
  phoneNumber1?: string;
  phoneNumber2?: string;
  website?: string;
  address: Address;
}

export interface Address {
  street?: string;
  number?: string;
  boxNumber?: string;
  postCode?: string;
  municipality?: string;
  province?: string;
  country?: string;
}
export interface Country {
  code: string;
  label: string;
}

export interface PostalCode {
  countryCode: string;
  postalCode: string;
  name: string;
}
