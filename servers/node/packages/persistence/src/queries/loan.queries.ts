/** Types generated for queries found in "src/queries/loan.sql" */
import { PreparedQuery } from '@pgtyped/runtime';

/** 'CreateLoan' parameters type */
export interface ICreateLoanParams {
  book_copy_id: number;
  member_id: number;
}

/** 'CreateLoan' return type */
export interface ICreateLoanResult {
  loan_id: number;
  loan_ident: string;
}

/** 'CreateLoan' query type */
export interface ICreateLoanQuery {
  params: ICreateLoanParams;
  result: ICreateLoanResult;
}

const createLoanIR: any = {"usedParamSet":{"book_copy_id":true,"member_id":true},"params":[{"name":"book_copy_id","required":true,"transform":{"type":"scalar"},"locs":[{"a":295,"b":308}]},{"name":"member_id","required":true,"transform":{"type":"scalar"},"locs":[{"a":311,"b":321}]}],"statement":"WITH next_id AS (\n  SELECT nextval(pg_get_serial_sequence('library.loan', 'loan_id'))::integer AS loan_id\n), inserted AS (\n  INSERT INTO library.loan (loan_id, loan_ident, book_copy_id, member_id)\n  OVERRIDING SYSTEM VALUE\n  SELECT next_id.loan_id, 'LN-' || lpad(next_id.loan_id::text, 6, '0'), :book_copy_id!, :member_id!\n  FROM next_id\n  RETURNING loan_id, loan_ident\n)\nSELECT loan_id, loan_ident\nFROM inserted"};

/**
 * Query generated from SQL:
 * ```
 * WITH next_id AS (
 *   SELECT nextval(pg_get_serial_sequence('library.loan', 'loan_id'))::integer AS loan_id
 * ), inserted AS (
 *   INSERT INTO library.loan (loan_id, loan_ident, book_copy_id, member_id)
 *   OVERRIDING SYSTEM VALUE
 *   SELECT next_id.loan_id, 'LN-' || lpad(next_id.loan_id::text, 6, '0'), :book_copy_id!, :member_id!
 *   FROM next_id
 *   RETURNING loan_id, loan_ident
 * )
 * SELECT loan_id, loan_ident
 * FROM inserted
 * ```
 */
export const createLoan = new PreparedQuery<ICreateLoanParams,ICreateLoanResult>(createLoanIR);


/** 'EndLoan' parameters type */
export interface IEndLoanParams {
  loan_id: number;
}

/** 'EndLoan' return type */
export type IEndLoanResult = void;

/** 'EndLoan' query type */
export interface IEndLoanQuery {
  params: IEndLoanParams;
  result: IEndLoanResult;
}

const endLoanIR: any = {"usedParamSet":{"loan_id":true},"params":[{"name":"loan_id","required":true,"transform":{"type":"scalar"},"locs":[{"a":72,"b":80}]}],"statement":"UPDATE library.loan\nSET dt_returned = CURRENT_TIMESTAMP\nWHERE loan_id = :loan_id!"};

/**
 * Query generated from SQL:
 * ```
 * UPDATE library.loan
 * SET dt_returned = CURRENT_TIMESTAMP
 * WHERE loan_id = :loan_id!
 * ```
 */
export const endLoan = new PreparedQuery<IEndLoanParams,IEndLoanResult>(endLoanIR);


/** 'FindActiveLoanByBookCopyId' parameters type */
export interface IFindActiveLoanByBookCopyIdParams {
  book_copy_id: number;
}

/** 'FindActiveLoanByBookCopyId' return type */
export interface IFindActiveLoanByBookCopyIdResult {
  book_copy_id: number;
  dt_created: Date;
  dt_due: Date | null;
  dt_modified: Date;
  dt_returned: Date | null;
  loan_id: number;
  loan_ident: string;
  member_id: number;
}

/** 'FindActiveLoanByBookCopyId' query type */
export interface IFindActiveLoanByBookCopyIdQuery {
  params: IFindActiveLoanByBookCopyIdParams;
  result: IFindActiveLoanByBookCopyIdResult;
}

const findActiveLoanByBookCopyIdIR: any = {"usedParamSet":{"book_copy_id":true},"params":[{"name":"book_copy_id","required":true,"transform":{"type":"scalar"},"locs":[{"a":260,"b":273}]}],"statement":"SELECT loan_id, loan_ident, dt_created, dt_modified, book_copy_id, member_id,\n  NULLIF(dt_due, '9999-01-01 00:00:00+00'::TIMESTAMPTZ) AS dt_due,\n  NULLIF(dt_returned, '9999-01-01 00:00:00+00'::TIMESTAMPTZ) AS dt_returned\nFROM library.loan\nWHERE book_copy_id = :book_copy_id!\n  AND dt_returned = '9999-01-01 00:00:00+00'::TIMESTAMPTZ\nORDER BY loan_id DESC\nLIMIT 1"};

/**
 * Query generated from SQL:
 * ```
 * SELECT loan_id, loan_ident, dt_created, dt_modified, book_copy_id, member_id,
 *   NULLIF(dt_due, '9999-01-01 00:00:00+00'::TIMESTAMPTZ) AS dt_due,
 *   NULLIF(dt_returned, '9999-01-01 00:00:00+00'::TIMESTAMPTZ) AS dt_returned
 * FROM library.loan
 * WHERE book_copy_id = :book_copy_id!
 *   AND dt_returned = '9999-01-01 00:00:00+00'::TIMESTAMPTZ
 * ORDER BY loan_id DESC
 * LIMIT 1
 * ```
 */
export const findActiveLoanByBookCopyId = new PreparedQuery<IFindActiveLoanByBookCopyIdParams,IFindActiveLoanByBookCopyIdResult>(findActiveLoanByBookCopyIdIR);


/** 'FindActiveLoanByBookCopyIdForUpdate' parameters type */
export interface IFindActiveLoanByBookCopyIdForUpdateParams {
  book_copy_id: number;
}

/** 'FindActiveLoanByBookCopyIdForUpdate' return type */
export interface IFindActiveLoanByBookCopyIdForUpdateResult {
  book_copy_id: number;
  dt_created: Date;
  dt_due: Date | null;
  dt_modified: Date;
  dt_returned: Date | null;
  loan_id: number;
  loan_ident: string;
  member_id: number;
}

/** 'FindActiveLoanByBookCopyIdForUpdate' query type */
export interface IFindActiveLoanByBookCopyIdForUpdateQuery {
  params: IFindActiveLoanByBookCopyIdForUpdateParams;
  result: IFindActiveLoanByBookCopyIdForUpdateResult;
}

const findActiveLoanByBookCopyIdForUpdateIR: any = {"usedParamSet":{"book_copy_id":true},"params":[{"name":"book_copy_id","required":true,"transform":{"type":"scalar"},"locs":[{"a":280,"b":293}]}],"statement":"SELECT l.loan_id, l.loan_ident, l.dt_created, l.dt_modified, l.book_copy_id, l.member_id,\n  NULLIF(l.dt_due, '9999-01-01 00:00:00+00'::TIMESTAMPTZ) AS dt_due,\n  NULLIF(l.dt_returned, '9999-01-01 00:00:00+00'::TIMESTAMPTZ) AS dt_returned\nFROM library.loan l\nWHERE l.book_copy_id = :book_copy_id!\n  AND l.dt_returned = '9999-01-01 00:00:00+00'::TIMESTAMPTZ\nORDER BY l.loan_id DESC\nLIMIT 1\nFOR UPDATE OF l"};

/**
 * Query generated from SQL:
 * ```
 * SELECT l.loan_id, l.loan_ident, l.dt_created, l.dt_modified, l.book_copy_id, l.member_id,
 *   NULLIF(l.dt_due, '9999-01-01 00:00:00+00'::TIMESTAMPTZ) AS dt_due,
 *   NULLIF(l.dt_returned, '9999-01-01 00:00:00+00'::TIMESTAMPTZ) AS dt_returned
 * FROM library.loan l
 * WHERE l.book_copy_id = :book_copy_id!
 *   AND l.dt_returned = '9999-01-01 00:00:00+00'::TIMESTAMPTZ
 * ORDER BY l.loan_id DESC
 * LIMIT 1
 * FOR UPDATE OF l
 * ```
 */
export const findActiveLoanByBookCopyIdForUpdate = new PreparedQuery<IFindActiveLoanByBookCopyIdForUpdateParams,IFindActiveLoanByBookCopyIdForUpdateResult>(findActiveLoanByBookCopyIdForUpdateIR);


/** 'CountActiveLoansByMemberId' parameters type */
export interface ICountActiveLoansByMemberIdParams {
  member_id: number;
}

/** 'CountActiveLoansByMemberId' return type */
export interface ICountActiveLoansByMemberIdResult {
  count: string | null;
}

/** 'CountActiveLoansByMemberId' query type */
export interface ICountActiveLoansByMemberIdQuery {
  params: ICountActiveLoansByMemberIdParams;
  result: ICountActiveLoansByMemberIdResult;
}

const countActiveLoansByMemberIdIR: any = {"usedParamSet":{"member_id":true},"params":[{"name":"member_id","required":true,"transform":{"type":"scalar"},"locs":[{"a":69,"b":79}]}],"statement":"SELECT COUNT(*)::BIGINT AS count\nFROM library.loan\nWHERE member_id = :member_id!\n  AND dt_returned = '9999-01-01 00:00:00+00'::TIMESTAMPTZ"};

/**
 * Query generated from SQL:
 * ```
 * SELECT COUNT(*)::BIGINT AS count
 * FROM library.loan
 * WHERE member_id = :member_id!
 *   AND dt_returned = '9999-01-01 00:00:00+00'::TIMESTAMPTZ
 * ```
 */
export const countActiveLoansByMemberId = new PreparedQuery<ICountActiveLoansByMemberIdParams,ICountActiveLoansByMemberIdResult>(countActiveLoansByMemberIdIR);


/** 'GetLoansByMemberIdent' parameters type */
export interface IGetLoansByMemberIdentParams {
  member_ident: string;
}

/** 'GetLoansByMemberIdent' return type */
export interface IGetLoansByMemberIdentResult {
  book_copy_id: number;
  dt_created: Date;
  dt_due: Date | null;
  dt_modified: Date;
  dt_returned: Date | null;
  loan_id: number;
  loan_ident: string;
  member_id: number;
}

/** 'GetLoansByMemberIdent' query type */
export interface IGetLoansByMemberIdentQuery {
  params: IGetLoansByMemberIdentParams;
  result: IGetLoansByMemberIdentResult;
}

const getLoansByMemberIdentIR: any = {"usedParamSet":{"member_ident":true},"params":[{"name":"member_ident","required":true,"transform":{"type":"scalar"},"locs":[{"a":331,"b":344}]}],"statement":"SELECT l.loan_id, l.loan_ident, l.dt_created, l.dt_modified, l.book_copy_id, l.member_id,\n  NULLIF(l.dt_due, '9999-01-01 00:00:00+00'::TIMESTAMPTZ) AS dt_due,\n  NULLIF(l.dt_returned, '9999-01-01 00:00:00+00'::TIMESTAMPTZ) AS dt_returned\nFROM library.loan l\nJOIN library.member m ON l.member_id = m.member_id\nWHERE m.member_ident = :member_ident!\nORDER BY l.dt_created DESC, l.loan_id DESC"};

/**
 * Query generated from SQL:
 * ```
 * SELECT l.loan_id, l.loan_ident, l.dt_created, l.dt_modified, l.book_copy_id, l.member_id,
 *   NULLIF(l.dt_due, '9999-01-01 00:00:00+00'::TIMESTAMPTZ) AS dt_due,
 *   NULLIF(l.dt_returned, '9999-01-01 00:00:00+00'::TIMESTAMPTZ) AS dt_returned
 * FROM library.loan l
 * JOIN library.member m ON l.member_id = m.member_id
 * WHERE m.member_ident = :member_ident!
 * ORDER BY l.dt_created DESC, l.loan_id DESC
 * ```
 */
export const getLoansByMemberIdent = new PreparedQuery<IGetLoansByMemberIdentParams,IGetLoansByMemberIdentResult>(getLoansByMemberIdentIR);


/** 'GetOverdueLoans' parameters type */
export type IGetOverdueLoansParams = void;

/** 'GetOverdueLoans' return type */
export interface IGetOverdueLoansResult {
  book_copy_id: number;
  dt_created: Date;
  dt_due: Date | null;
  dt_modified: Date;
  dt_returned: Date | null;
  loan_id: number;
  loan_ident: string;
  member_id: number;
}

/** 'GetOverdueLoans' query type */
export interface IGetOverdueLoansQuery {
  params: IGetOverdueLoansParams;
  result: IGetOverdueLoansResult;
}

const getOverdueLoansIR: any = {"usedParamSet":{},"params":[],"statement":"SELECT loan_id, loan_ident, dt_created, dt_modified, book_copy_id, member_id,\n  NULLIF(dt_due, '9999-01-01 00:00:00+00'::TIMESTAMPTZ) AS dt_due,\n  NULLIF(dt_returned, '9999-01-01 00:00:00+00'::TIMESTAMPTZ) AS dt_returned\nFROM library.loan\nWHERE dt_returned = '9999-01-01 00:00:00+00'::TIMESTAMPTZ\n  AND dt_due < CURRENT_TIMESTAMP\nORDER BY dt_due, loan_id"};

/**
 * Query generated from SQL:
 * ```
 * SELECT loan_id, loan_ident, dt_created, dt_modified, book_copy_id, member_id,
 *   NULLIF(dt_due, '9999-01-01 00:00:00+00'::TIMESTAMPTZ) AS dt_due,
 *   NULLIF(dt_returned, '9999-01-01 00:00:00+00'::TIMESTAMPTZ) AS dt_returned
 * FROM library.loan
 * WHERE dt_returned = '9999-01-01 00:00:00+00'::TIMESTAMPTZ
 *   AND dt_due < CURRENT_TIMESTAMP
 * ORDER BY dt_due, loan_id
 * ```
 */
export const getOverdueLoans = new PreparedQuery<IGetOverdueLoansParams,IGetOverdueLoansResult>(getOverdueLoansIR);


