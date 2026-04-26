import type { Pool, PoolClient } from "pg";

import type {
  BookCopy,
  BookCopyId,
  BookCopyPrepared,
  BookCopyReadRepository,
  BookCopyStatus,
  BookCopyWriteRepository
} from "@library/domain";

import { mapBookCopy } from "./mappers.js";
import {
  createBookCopy,
  getBookCopyByBarcode,
  getBookCopyByBarcodeForUpdate,
  getBookCopyById,
  updateBookCopyStatus
} from "./queries/book-copy.queries.js";

export class BookCopyReadRepositoryPostgres implements BookCopyReadRepository {
  constructor(private readonly pool: Pool) {}

  async getById(id: BookCopyId): Promise<BookCopy | null> {
    const rows = await getBookCopyById.run({ book_copy_id: id }, this.pool);
    return rows[0] === undefined ? null : mapBookCopy(rows[0]);
  }

  async getByBarcode(barcode: string): Promise<BookCopy | null> {
    const rows = await getBookCopyByBarcode.run({ barcode }, this.pool);
    return rows[0] === undefined ? null : mapBookCopy(rows[0]);
  }
}

export class BookCopyWriteRepositoryPostgres implements BookCopyWriteRepository {
  constructor(private readonly client: PoolClient) {}

  async create(insert: BookCopyPrepared): Promise<BookCopy> {
    const rows = await createBookCopy.run(
      {
        book_id: insert.book_id,
        status: insert.status,
        barcode: insert.barcode
      },
      this.client
    );
    const created = rows[0];
    if (created === undefined) throw new Error("Failed to create book copy");

    const now = new Date();
    return {
      id: created.book_copy_id,
      barcode: insert.barcode,
      dt_created: now,
      dt_modified: now,
      book_id: insert.book_id,
      status: insert.status
    };
  }

  async getByBarcodeForUpdate(barcode: string): Promise<BookCopy | null> {
    const rows = await getBookCopyByBarcodeForUpdate.run({ barcode }, this.client);
    return rows[0] === undefined ? null : mapBookCopy(rows[0]);
  }

  async updateStatus(id: BookCopyId, status: BookCopyStatus): Promise<void> {
    await updateBookCopyStatus.run({ book_copy_id: id, status }, this.client);
  }
}
