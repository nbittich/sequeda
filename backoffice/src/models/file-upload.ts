export interface FileUpload {
  _id: string;
  creationDate?: Date;
  updatedDate?: Date;
  thumbnailId?: string;
  contentType?: string;
  originalFilename?: string;
  extension?: string;
  size?: number;
  publicResource?: number;
  correlationId?: number;
}
