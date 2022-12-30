export interface Pageable {
  page: number;
  limit: number;
  sort?: any;
}

export function toQueryString(pageable: Pageable): string {
  return `page=${pageable.page}&limit=${pageable.limit}${
    pageable.sort ? '&sort=' + JSON.stringify(pageable.sort) : ''
  }`;
}

export interface Page<T> {
  totalElements: number;
  currentPage: number;
  nextPage?: number;
  pageSize: number;
  content: T[];
}

export type PaginationProp = {
  pagination: { page?: number; rowsPerPage?: number; rowsNumber?: number };
};
