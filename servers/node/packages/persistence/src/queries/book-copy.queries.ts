/** Types generated for queries found in "src/queries/book-copy.sql" */
import { PreparedQuery } from '@pgtyped/runtime';

/** 'CreateBookCopy' parameters type */
export interface ICreateBookCopyParams {
  barcode: string;
  book_id: number;
  status: string;
}

/** 'CreateBookCopy' return type */
export interface ICreateBookCopyResult {
  book_copy_id: number;
}

/** 'CreateBookCopy' query type */
export interface ICreateBookCopyQuery {
  params: ICreateBookCopyParams;
  result: ICreateBookCopyResult;
}

const createBookCopyIR: any = {"usedParamSet":{"book_id":true,"status":true,"barcode":true},"params":[{"name":"book_id","required":true,"transform":{"type":"scalar"},"locs":[{"a":71,"b":79}]},{"name":"status","required":true,"transform":{"type":"scalar"},"locs":[{"a":209,"b":216}]},{"name":"barcode","required":true,"transform":{"type":"scalar"},"locs":[{"a":225,"b":233}]}],"statement":"INSERT INTO library.book_copy (book_id, status_id, barcode)\nVALUES (\n  :book_id!,\n  (\n    SELECT struct_type_id\n    FROM library.struct_type\n    WHERE group_name = 'book_copy_status'\n      AND att_pub_ident = :status!\n  ),\n  :barcode!\n)\nRETURNING book_copy_id"};

/**
 * Query generated from SQL:
 * ```
 * INSERT INTO library.book_copy (book_id, status_id, barcode)
 * VALUES (
 *   :book_id!,
 *   (
 *     SELECT struct_type_id
 *     FROM library.struct_type
 *     WHERE group_name = 'book_copy_status'
 *       AND att_pub_ident = :status!
 *   ),
 *   :barcode!
 * )
 * RETURNING book_copy_id
 * ```
 */
export const createBookCopy = new PreparedQuery<ICreateBookCopyParams,ICreateBookCopyResult>(createBookCopyIR);


/** 'GetBookCopyById' parameters type */
export interface IGetBookCopyByIdParams {
  book_copy_id: number;
}

/** 'GetBookCopyById' return type */
export interface IGetBookCopyByIdResult {
  barcode: string;
  book_copy_id: number;
  book_id: number;
  dt_created: Date;
  dt_modified: Date;
  status: string;
}

/** 'GetBookCopyById' query type */
export interface IGetBookCopyByIdQuery {
  params: IGetBookCopyByIdParams;
  result: IGetBookCopyByIdResult;
}

const getBookCopyByIdIR: any = {"usedParamSet":{"book_copy_id":true},"params":[{"name":"book_copy_id","required":true,"transform":{"type":"scalar"},"locs":[{"a":220,"b":233}]}],"statement":"SELECT bc.book_copy_id, bc.barcode, bc.dt_created, bc.dt_modified, bc.book_id, st.att_pub_ident AS status\nFROM library.book_copy bc\nJOIN library.struct_type st ON bc.status_id = st.struct_type_id\nWHERE bc.book_copy_id = :book_copy_id!"};

/**
 * Query generated from SQL:
 * ```
 * SELECT bc.book_copy_id, bc.barcode, bc.dt_created, bc.dt_modified, bc.book_id, st.att_pub_ident AS status
 * FROM library.book_copy bc
 * JOIN library.struct_type st ON bc.status_id = st.struct_type_id
 * WHERE bc.book_copy_id = :book_copy_id!
 * ```
 */
export const getBookCopyById = new PreparedQuery<IGetBookCopyByIdParams,IGetBookCopyByIdResult>(getBookCopyByIdIR);


/** 'GetBookCopyByBarcode' parameters type */
export interface IGetBookCopyByBarcodeParams {
  barcode: string;
}

/** 'GetBookCopyByBarcode' return type */
export interface IGetBookCopyByBarcodeResult {
  barcode: string;
  book_copy_id: number;
  book_id: number;
  dt_created: Date;
  dt_modified: Date;
  status: string;
}

/** 'GetBookCopyByBarcode' query type */
export interface IGetBookCopyByBarcodeQuery {
  params: IGetBookCopyByBarcodeParams;
  result: IGetBookCopyByBarcodeResult;
}

const getBookCopyByBarcodeIR: any = {"usedParamSet":{"barcode":true},"params":[{"name":"barcode","required":true,"transform":{"type":"scalar"},"locs":[{"a":215,"b":223}]}],"statement":"SELECT bc.book_copy_id, bc.barcode, bc.dt_created, bc.dt_modified, bc.book_id, st.att_pub_ident AS status\nFROM library.book_copy bc\nJOIN library.struct_type st ON bc.status_id = st.struct_type_id\nWHERE bc.barcode = :barcode!"};

/**
 * Query generated from SQL:
 * ```
 * SELECT bc.book_copy_id, bc.barcode, bc.dt_created, bc.dt_modified, bc.book_id, st.att_pub_ident AS status
 * FROM library.book_copy bc
 * JOIN library.struct_type st ON bc.status_id = st.struct_type_id
 * WHERE bc.barcode = :barcode!
 * ```
 */
export const getBookCopyByBarcode = new PreparedQuery<IGetBookCopyByBarcodeParams,IGetBookCopyByBarcodeResult>(getBookCopyByBarcodeIR);


/** 'GetBookCopyByBarcodeForUpdate' parameters type */
export interface IGetBookCopyByBarcodeForUpdateParams {
  barcode: string;
}

/** 'GetBookCopyByBarcodeForUpdate' return type */
export interface IGetBookCopyByBarcodeForUpdateResult {
  barcode: string;
  book_copy_id: number;
  book_id: number;
  dt_created: Date;
  dt_modified: Date;
  status: string;
}

/** 'GetBookCopyByBarcodeForUpdate' query type */
export interface IGetBookCopyByBarcodeForUpdateQuery {
  params: IGetBookCopyByBarcodeForUpdateParams;
  result: IGetBookCopyByBarcodeForUpdateResult;
}

const getBookCopyByBarcodeForUpdateIR: any = {"usedParamSet":{"barcode":true},"params":[{"name":"barcode","required":true,"transform":{"type":"scalar"},"locs":[{"a":215,"b":223}]}],"statement":"SELECT bc.book_copy_id, bc.barcode, bc.dt_created, bc.dt_modified, bc.book_id, st.att_pub_ident AS status\nFROM library.book_copy bc\nJOIN library.struct_type st ON bc.status_id = st.struct_type_id\nWHERE bc.barcode = :barcode!\nFOR UPDATE OF bc"};

/**
 * Query generated from SQL:
 * ```
 * SELECT bc.book_copy_id, bc.barcode, bc.dt_created, bc.dt_modified, bc.book_id, st.att_pub_ident AS status
 * FROM library.book_copy bc
 * JOIN library.struct_type st ON bc.status_id = st.struct_type_id
 * WHERE bc.barcode = :barcode!
 * FOR UPDATE OF bc
 * ```
 */
export const getBookCopyByBarcodeForUpdate = new PreparedQuery<IGetBookCopyByBarcodeForUpdateParams,IGetBookCopyByBarcodeForUpdateResult>(getBookCopyByBarcodeForUpdateIR);


/** 'UpdateBookCopyStatus' parameters type */
export interface IUpdateBookCopyStatusParams {
  book_copy_id: number;
  status: string;
}

/** 'UpdateBookCopyStatus' return type */
export type IUpdateBookCopyStatusResult = void;

/** 'UpdateBookCopyStatus' query type */
export interface IUpdateBookCopyStatusQuery {
  params: IUpdateBookCopyStatusParams;
  result: IUpdateBookCopyStatusResult;
}

const updateBookCopyStatusIR: any = {"usedParamSet":{"status":true,"book_copy_id":true},"params":[{"name":"status","required":true,"transform":{"type":"scalar"},"locs":[{"a":158,"b":165}]},{"name":"book_copy_id","required":true,"transform":{"type":"scalar"},"locs":[{"a":190,"b":203}]}],"statement":"UPDATE library.book_copy\nSET status_id = (\n  SELECT struct_type_id\n  FROM library.struct_type\n  WHERE group_name = 'book_copy_status'\n    AND att_pub_ident = :status!\n)\nWHERE book_copy_id = :book_copy_id!"};

/**
 * Query generated from SQL:
 * ```
 * UPDATE library.book_copy
 * SET status_id = (
 *   SELECT struct_type_id
 *   FROM library.struct_type
 *   WHERE group_name = 'book_copy_status'
 *     AND att_pub_ident = :status!
 * )
 * WHERE book_copy_id = :book_copy_id!
 * ```
 */
export const updateBookCopyStatus = new PreparedQuery<IUpdateBookCopyStatusParams,IUpdateBookCopyStatusResult>(updateBookCopyStatusIR);


