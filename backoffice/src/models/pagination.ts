
export interface Pageable {
  page: number,
  limit: number,
  sort?: any,
}

export interface Page<T> {
  totalElements: number,
  currentPage: number,
  nextPage?: number,
  pageSize: number,
  content: T[],
}
