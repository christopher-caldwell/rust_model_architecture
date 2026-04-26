import type { Pool, PoolClient } from "pg";

import type {
  Member,
  MemberId,
  MemberIdent,
  MemberPrepared,
  MemberReadRepository,
  MemberStatus,
  MemberWriteRepository
} from "@library/domain";

import { mapMember } from "./mappers.js";
import {
  createMember,
  getMemberById,
  getMemberByIdent,
  getMemberByIdentForUpdate,
  updateMemberStatus
} from "./queries/member.queries.js";

export class MemberReadRepositoryPostgres implements MemberReadRepository {
  constructor(private readonly pool: Pool) {}

  async getById(id: MemberId): Promise<Member | null> {
    const rows = await getMemberById.run({ member_id: id }, this.pool);
    return rows[0] === undefined ? null : mapMember(rows[0]);
  }

  async getByIdent(ident: MemberIdent): Promise<Member | null> {
    const rows = await getMemberByIdent.run({ member_ident: ident }, this.pool);
    return rows[0] === undefined ? null : mapMember(rows[0]);
  }
}

export class MemberWriteRepositoryPostgres implements MemberWriteRepository {
  constructor(private readonly client: PoolClient) {}

  async create(insert: MemberPrepared): Promise<Member> {
    const rows = await createMember.run(
      {
        member_ident: insert.ident,
        status: insert.status,
        full_name: insert.full_name,
        max_active_loans: insert.max_active_loans
      },
      this.client
    );
    const created = rows[0];
    if (created === undefined) throw new Error("Failed to create member");

    const now = new Date();
    return {
      id: created.member_id,
      ident: insert.ident,
      dt_created: now,
      dt_modified: now,
      status: insert.status,
      full_name: insert.full_name,
      max_active_loans: insert.max_active_loans
    };
  }

  async getByIdentForUpdate(ident: MemberIdent): Promise<Member | null> {
    const rows = await getMemberByIdentForUpdate.run({ member_ident: ident }, this.client);
    return rows[0] === undefined ? null : mapMember(rows[0]);
  }

  async updateStatus(id: MemberId, status: MemberStatus): Promise<void> {
    await updateMemberStatus.run({ member_id: id, status }, this.client);
  }
}
