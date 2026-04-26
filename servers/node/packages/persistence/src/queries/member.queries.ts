/** Types generated for queries found in "src/queries/member.sql" */
import { PreparedQuery } from '@pgtyped/runtime';

/** 'CreateMember' parameters type */
export interface ICreateMemberParams {
  full_name: string;
  max_active_loans: number;
  member_ident: string;
  status: string;
}

/** 'CreateMember' return type */
export interface ICreateMemberResult {
  member_id: number;
}

/** 'CreateMember' query type */
export interface ICreateMemberQuery {
  params: ICreateMemberParams;
  result: ICreateMemberResult;
}

const createMemberIR: any = {"usedParamSet":{"member_ident":true,"status":true,"full_name":true,"max_active_loans":true},"params":[{"name":"member_ident","required":true,"transform":{"type":"scalar"},"locs":[{"a":93,"b":106}]},{"name":"status","required":true,"transform":{"type":"scalar"},"locs":[{"a":233,"b":240}]},{"name":"full_name","required":true,"transform":{"type":"scalar"},"locs":[{"a":249,"b":259}]},{"name":"max_active_loans","required":true,"transform":{"type":"scalar"},"locs":[{"a":264,"b":281}]}],"statement":"INSERT INTO library.member (member_ident, status_id, full_name, max_active_loans)\nVALUES (\n  :member_ident!,\n  (\n    SELECT struct_type_id\n    FROM library.struct_type\n    WHERE group_name = 'member_status'\n      AND att_pub_ident = :status!\n  ),\n  :full_name!,\n  :max_active_loans!\n)\nRETURNING member_id"};

/**
 * Query generated from SQL:
 * ```
 * INSERT INTO library.member (member_ident, status_id, full_name, max_active_loans)
 * VALUES (
 *   :member_ident!,
 *   (
 *     SELECT struct_type_id
 *     FROM library.struct_type
 *     WHERE group_name = 'member_status'
 *       AND att_pub_ident = :status!
 *   ),
 *   :full_name!,
 *   :max_active_loans!
 * )
 * RETURNING member_id
 * ```
 */
export const createMember = new PreparedQuery<ICreateMemberParams,ICreateMemberResult>(createMemberIR);


/** 'GetMemberById' parameters type */
export interface IGetMemberByIdParams {
  member_id: number;
}

/** 'GetMemberById' return type */
export interface IGetMemberByIdResult {
  dt_created: Date;
  dt_modified: Date;
  full_name: string;
  max_active_loans: number;
  member_id: number;
  member_ident: string;
  status: string;
}

/** 'GetMemberById' query type */
export interface IGetMemberByIdQuery {
  params: IGetMemberByIdParams;
  result: IGetMemberByIdResult;
}

const getMemberByIdIR: any = {"usedParamSet":{"member_id":true},"params":[{"name":"member_id","required":true,"transform":{"type":"scalar"},"locs":[{"a":230,"b":240}]}],"statement":"SELECT m.member_id, m.member_ident, m.dt_created, m.dt_modified, st.att_pub_ident AS status, m.full_name, m.max_active_loans\nFROM library.member m\nJOIN library.struct_type st ON m.status_id = st.struct_type_id\nWHERE m.member_id = :member_id!\n  AND st.group_name = 'member_status'\n  AND st.att_pub_ident IN ('active', 'suspended')"};

/**
 * Query generated from SQL:
 * ```
 * SELECT m.member_id, m.member_ident, m.dt_created, m.dt_modified, st.att_pub_ident AS status, m.full_name, m.max_active_loans
 * FROM library.member m
 * JOIN library.struct_type st ON m.status_id = st.struct_type_id
 * WHERE m.member_id = :member_id!
 *   AND st.group_name = 'member_status'
 *   AND st.att_pub_ident IN ('active', 'suspended')
 * ```
 */
export const getMemberById = new PreparedQuery<IGetMemberByIdParams,IGetMemberByIdResult>(getMemberByIdIR);


/** 'GetMemberByIdent' parameters type */
export interface IGetMemberByIdentParams {
  member_ident: string;
}

/** 'GetMemberByIdent' return type */
export interface IGetMemberByIdentResult {
  dt_created: Date;
  dt_modified: Date;
  full_name: string;
  max_active_loans: number;
  member_id: number;
  member_ident: string;
  status: string;
}

/** 'GetMemberByIdent' query type */
export interface IGetMemberByIdentQuery {
  params: IGetMemberByIdentParams;
  result: IGetMemberByIdentResult;
}

const getMemberByIdentIR: any = {"usedParamSet":{"member_ident":true},"params":[{"name":"member_ident","required":true,"transform":{"type":"scalar"},"locs":[{"a":233,"b":246}]}],"statement":"SELECT m.member_id, m.member_ident, m.dt_created, m.dt_modified, st.att_pub_ident AS status, m.full_name, m.max_active_loans\nFROM library.member m\nJOIN library.struct_type st ON m.status_id = st.struct_type_id\nWHERE m.member_ident = :member_ident!\n  AND st.group_name = 'member_status'\n  AND st.att_pub_ident IN ('active', 'suspended')"};

/**
 * Query generated from SQL:
 * ```
 * SELECT m.member_id, m.member_ident, m.dt_created, m.dt_modified, st.att_pub_ident AS status, m.full_name, m.max_active_loans
 * FROM library.member m
 * JOIN library.struct_type st ON m.status_id = st.struct_type_id
 * WHERE m.member_ident = :member_ident!
 *   AND st.group_name = 'member_status'
 *   AND st.att_pub_ident IN ('active', 'suspended')
 * ```
 */
export const getMemberByIdent = new PreparedQuery<IGetMemberByIdentParams,IGetMemberByIdentResult>(getMemberByIdentIR);


/** 'GetMemberByIdentForUpdate' parameters type */
export interface IGetMemberByIdentForUpdateParams {
  member_ident: string;
}

/** 'GetMemberByIdentForUpdate' return type */
export interface IGetMemberByIdentForUpdateResult {
  dt_created: Date;
  dt_modified: Date;
  full_name: string;
  max_active_loans: number;
  member_id: number;
  member_ident: string;
  status: string;
}

/** 'GetMemberByIdentForUpdate' query type */
export interface IGetMemberByIdentForUpdateQuery {
  params: IGetMemberByIdentForUpdateParams;
  result: IGetMemberByIdentForUpdateResult;
}

const getMemberByIdentForUpdateIR: any = {"usedParamSet":{"member_ident":true},"params":[{"name":"member_ident","required":true,"transform":{"type":"scalar"},"locs":[{"a":233,"b":246}]}],"statement":"SELECT m.member_id, m.member_ident, m.dt_created, m.dt_modified, st.att_pub_ident AS status, m.full_name, m.max_active_loans\nFROM library.member m\nJOIN library.struct_type st ON m.status_id = st.struct_type_id\nWHERE m.member_ident = :member_ident!\n  AND st.group_name = 'member_status'\n  AND st.att_pub_ident IN ('active', 'suspended')\nFOR UPDATE OF m"};

/**
 * Query generated from SQL:
 * ```
 * SELECT m.member_id, m.member_ident, m.dt_created, m.dt_modified, st.att_pub_ident AS status, m.full_name, m.max_active_loans
 * FROM library.member m
 * JOIN library.struct_type st ON m.status_id = st.struct_type_id
 * WHERE m.member_ident = :member_ident!
 *   AND st.group_name = 'member_status'
 *   AND st.att_pub_ident IN ('active', 'suspended')
 * FOR UPDATE OF m
 * ```
 */
export const getMemberByIdentForUpdate = new PreparedQuery<IGetMemberByIdentForUpdateParams,IGetMemberByIdentForUpdateResult>(getMemberByIdentForUpdateIR);


/** 'UpdateMemberStatus' parameters type */
export interface IUpdateMemberStatusParams {
  member_id: number;
  status: string;
}

/** 'UpdateMemberStatus' return type */
export type IUpdateMemberStatusResult = void;

/** 'UpdateMemberStatus' query type */
export interface IUpdateMemberStatusQuery {
  params: IUpdateMemberStatusParams;
  result: IUpdateMemberStatusResult;
}

const updateMemberStatusIR: any = {"usedParamSet":{"status":true,"member_id":true},"params":[{"name":"status","required":true,"transform":{"type":"scalar"},"locs":[{"a":152,"b":159}]},{"name":"member_id","required":true,"transform":{"type":"scalar"},"locs":[{"a":181,"b":191}]}],"statement":"UPDATE library.member\nSET status_id = (\n  SELECT struct_type_id\n  FROM library.struct_type\n  WHERE group_name = 'member_status'\n    AND att_pub_ident = :status!\n)\nWHERE member_id = :member_id!"};

/**
 * Query generated from SQL:
 * ```
 * UPDATE library.member
 * SET status_id = (
 *   SELECT struct_type_id
 *   FROM library.struct_type
 *   WHERE group_name = 'member_status'
 *     AND att_pub_ident = :status!
 * )
 * WHERE member_id = :member_id!
 * ```
 */
export const updateMemberStatus = new PreparedQuery<IUpdateMemberStatusParams,IUpdateMemberStatusResult>(updateMemberStatusIR);


