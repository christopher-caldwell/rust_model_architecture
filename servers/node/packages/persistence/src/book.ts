import type { Pool, PoolClient } from "pg";

import type { Book, BookPrepared, BookReadRepository, BookWriteRepository } from "@library/domain";

import { mapBook } from "./mappers.js";
import { createBook, getBookByIsbn, getBookCatalog } from "./queries/book.queries.js";

export class BookReadRepositoryPostgres implements BookReadRepository {
  constructor(private readonly pool: Pool) {}

  async getCatalog(): Promise<Book[]> {
    const rows = await getBookCatalog.run(undefined, this.pool);
    return rows.map(mapBook);
  }

  async getByIsbn(isbn: string): Promise<Book | null> {
    const rows = await getBookByIsbn.run({ isbn }, this.pool);
    return rows[0] === undefined ? null : mapBook(rows[0]);
  }
}

export class BookWriteRepositoryPostgres implements BookWriteRepository {
  constructor(private readonly client: PoolClient) {}

  async create(insert: BookPrepared): Promise<Book> {
    const rows = await createBook.run(
      {
        isbn: insert.isbn,
        title: insert.title,
        author_name: insert.author_name
      },
      this.client
    );
    const created = rows[0];
    if (created === undefined) throw new Error("Failed to create book");

    const now = new Date();
    return {
      id: created.book_id,
      isbn: insert.isbn,
      dt_created: now,
      dt_modified: now,
      title: insert.title,
      author_name: insert.author_name
    };
  }

  async getByIsbn(isbn: string): Promise<Book | null> {
    const rows = await getBookByIsbn.run({ isbn }, this.client);
    return rows[0] === undefined ? null : mapBook(rows[0]);
  }
}
