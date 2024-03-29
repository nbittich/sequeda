export interface Product {
  _id?: string;
  label: string;
  name: string;
  mainPictureId?: string;
  description?: string;
  tags?: string[];
  pricePerUnit: number;
  vat: number;
  unitType?: string;
  creationDate?: Date;
  updatedDate?: Date;
}
