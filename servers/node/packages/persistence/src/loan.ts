import type { Pool, PoolClient } from "pg";

import type {
  BookCopyId,
  Loan,
  LoanId,
  LoanPrepared,
  LoanReadRepository,
  LoanWriteRepository,
  MemberId,
  MemberIdent
} from "@library/domain";

import { mapLoan } from "./mappers.js";
import {
  countActiveLoansByMemberId,
  createLoan,
  endLoan,
  findActiveLoanByBookCopyId,
  findActiveLoanByBookCopyIdForUpdate,
  getLoansByMemberIdent,
  getOverdueLoans
} from "./queries/loan.queries.js";

export class LoanReadRepositoryPostgres implements LoanReadRepository {
  constructor(private readonly pool: Pool) {}

  async getByMemberIdent(ident: MemberIdent): Promise<Loan[]> {
    const rows = await getLoansByMemberIdent.run({ member_ident: ident }, this.pool);
    return rows.map(mapLoan);
  }

  async getOverdue(): Promise<Loan[]> {
    const rows = await getOverdueLoans.run(undefined, this.pool);
    return rows.map(mapLoan);
  }

  async findActiveByBookCopyId(id: BookCopyId): Promise<Loan | null> {
    const rows = await findActiveLoanByBookCopyId.run({ book_copy_id: id }, this.pool);
    return rows[0] === undefined ? null : mapLoan(rows[0]);
  }

  async countActiveByMemberId(id: MemberId): Promise<number> {
    const rows = await countActiveLoansByMemberId.run({ member_id: id }, this.pool);
    const row = rows[0];
    return row === undefined ? 0 : Number(row.count);
  }
}

export class LoanWriteRepositoryPostgres implements LoanWriteRepository {
  constructor(private readonly client: PoolClient) {}

  async create(insert: LoanPrepared): Promise<Loan> {
    const rows = await createLoan.run(
      {
        book_copy_id: insert.book_copy_id,
        member_id: insert.member_id
      },
      this.client
    );
    const created = rows[0];
    if (created === undefined) throw new Error("Failed to create loan");

    const now = new Date();
    return {
      id: created.loan_id,
      ident: created.loan_ident,
      dt_created: now,
      dt_modified: now,
      book_copy_id: insert.book_copy_id,
      member_id: insert.member_id,
      dt_due: null,
      dt_returned: null
    };
  }

  async end(id: LoanId): Promise<void> {
    await endLoan.run({ loan_id: id }, this.client);
  }

  async findActiveByBookCopyIdForUpdate(id: BookCopyId): Promise<Loan | null> {
    const rows = await findActiveLoanByBookCopyIdForUpdate.run({ book_copy_id: id }, this.client);
    return rows[0] === undefined ? null : mapLoan(rows[0]);
  }

  async countActiveByMemberId(id: MemberId): Promise<number> {
    const rows = await countActiveLoansByMemberId.run({ member_id: id }, this.client);
    const row = rows[0];
    return row === undefined ? 0 : Number(row.count);
  }
}
