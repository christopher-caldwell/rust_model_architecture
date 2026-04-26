/** Types generated for queries found in "src/queries/book.sql" */
import { PreparedQuery } from '@pgtyped/runtime';

/** 'CreateBook' parameters type */
export interface ICreateBookParams {
  author_name: string;
  isbn: string;
  title: string;
}

/** 'CreateBook' return type */
export interface ICreateBookResult {
  book_id: number;
}

/** 'CreateBook' query type */
export interface ICreateBookQuery {
  params: ICreateBookParams;
  result: ICreateBookResult;
}

const createBookIR: any = {"usedParamSet":{"isbn":true,"title":true,"author_name":true},"params":[{"name":"isbn","required":true,"transform":{"type":"scalar"},"locs":[{"a":60,"b":65}]},{"name":"title","required":true,"transform":{"type":"scalar"},"locs":[{"a":68,"b":74}]},{"name":"author_name","required":true,"transform":{"type":"scalar"},"locs":[{"a":77,"b":89}]}],"statement":"INSERT INTO library.book (isbn, title, author_name)\nVALUES (:isbn!, :title!, :author_name!)\nRETURNING book_id"};

/**
 * Query generated from SQL:
 * ```
 * INSERT INTO library.book (isbn, title, author_name)
 * VALUES (:isbn!, :title!, :author_name!)
 * RETURNING book_id
 * ```
 */
export const createBook = new PreparedQuery<ICreateBookParams,ICreateBookResult>(createBookIR);


/** 'GetBookByIsbn' parameters type */
export interface IGetBookByIsbnParams {
  isbn: string;
}

/** 'GetBookByIsbn' return type */
export interface IGetBookByIsbnResult {
  author_name: string;
  book_id: number;
  dt_created: Date;
  dt_modified: Date;
  isbn: string;
  title: string;
}

/** 'GetBookByIsbn' query type */
export interface IGetBookByIsbnQuery {
  params: IGetBookByIsbnParams;
  result: IGetBookByIsbnResult;
}

const getBookByIsbnIR: any = {"usedParamSet":{"isbn":true},"params":[{"name":"isbn","required":true,"transform":{"type":"scalar"},"locs":[{"a":97,"b":102}]}],"statement":"SELECT book_id, isbn, dt_created, dt_modified, title, author_name\nFROM library.book\nWHERE isbn = :isbn!"};

/**
 * Query generated from SQL:
 * ```
 * SELECT book_id, isbn, dt_created, dt_modified, title, author_name
 * FROM library.book
 * WHERE isbn = :isbn!
 * ```
 */
export const getBookByIsbn = new PreparedQuery<IGetBookByIsbnParams,IGetBookByIsbnResult>(getBookByIsbnIR);


/** 'GetBookCatalog' parameters type */
export type IGetBookCatalogParams = void;

/** 'GetBookCatalog' return type */
export interface IGetBookCatalogResult {
  author_name: string;
  book_id: number;
  dt_created: Date;
  dt_modified: Date;
  isbn: string;
  title: string;
}

/** 'GetBookCatalog' query type */
export interface IGetBookCatalogQuery {
  params: IGetBookCatalogParams;
  result: IGetBookCatalogResult;
}

const getBookCatalogIR: any = {"usedParamSet":{},"params":[],"statement":"SELECT book_id, isbn, dt_created, dt_modified, title, author_name\nFROM library.book\nORDER BY title, book_id"};

/**
 * Query generated from SQL:
 * ```
 * SELECT book_id, isbn, dt_created, dt_modified, title, author_name
 * FROM library.book
 * ORDER BY title, book_id
 * ```
 */
export const getBookCatalog = new PreparedQuery<IGetBookCatalogParams,IGetBookCatalogResult>(getBookCatalogIR);


