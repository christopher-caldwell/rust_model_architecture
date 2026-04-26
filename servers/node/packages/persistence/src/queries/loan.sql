/* @name CreateLoan */
WITH next_id AS (
  SELECT nextval(pg_get_serial_sequence('library.loan', 'loan_id'))::integer AS loan_id
), inserted AS (
  INSERT INTO library.loan (loan_id, loan_ident, book_copy_id, member_id)
  OVERRIDING SYSTEM VALUE
  SELECT next_id.loan_id, 'LN-' || lpad(next_id.loan_id::text, 6, '0'), :book_copy_id!, :member_id!
  FROM next_id
  RETURNING loan_id, loan_ident
)
SELECT loan_id, loan_ident
FROM inserted;

/* @name EndLoan */
UPDATE library.loan
SET dt_returned = CURRENT_TIMESTAMP
WHERE loan_id = :loan_id!;

/* @name FindActiveLoanByBookCopyId */
SELECT loan_id, loan_ident, dt_created, dt_modified, book_copy_id, member_id,
  NULLIF(dt_due, '9999-01-01 00:00:00+00'::TIMESTAMPTZ) AS dt_due,
  NULLIF(dt_returned, '9999-01-01 00:00:00+00'::TIMESTAMPTZ) AS dt_returned
FROM library.loan
WHERE book_copy_id = :book_copy_id!
  AND dt_returned = '9999-01-01 00:00:00+00'::TIMESTAMPTZ
ORDER BY loan_id DESC
LIMIT 1;

/* @name FindActiveLoanByBookCopyIdForUpdate */
SELECT l.loan_id, l.loan_ident, l.dt_created, l.dt_modified, l.book_copy_id, l.member_id,
  NULLIF(l.dt_due, '9999-01-01 00:00:00+00'::TIMESTAMPTZ) AS dt_due,
  NULLIF(l.dt_returned, '9999-01-01 00:00:00+00'::TIMESTAMPTZ) AS dt_returned
FROM library.loan l
WHERE l.book_copy_id = :book_copy_id!
  AND l.dt_returned = '9999-01-01 00:00:00+00'::TIMESTAMPTZ
ORDER BY l.loan_id DESC
LIMIT 1
FOR UPDATE OF l;

/* @name CountActiveLoansByMemberId */
SELECT COUNT(*)::BIGINT AS count
FROM library.loan
WHERE member_id = :member_id!
  AND dt_returned = '9999-01-01 00:00:00+00'::TIMESTAMPTZ;

/* @name GetLoansByMemberIdent */
SELECT l.loan_id, l.loan_ident, l.dt_created, l.dt_modified, l.book_copy_id, l.member_id,
  NULLIF(l.dt_due, '9999-01-01 00:00:00+00'::TIMESTAMPTZ) AS dt_due,
  NULLIF(l.dt_returned, '9999-01-01 00:00:00+00'::TIMESTAMPTZ) AS dt_returned
FROM library.loan l
JOIN library.member m ON l.member_id = m.member_id
WHERE m.member_ident = :member_ident!
ORDER BY l.dt_created DESC, l.loan_id DESC;

/* @name GetOverdueLoans */
SELECT loan_id, loan_ident, dt_created, dt_modified, book_copy_id, member_id,
  NULLIF(dt_due, '9999-01-01 00:00:00+00'::TIMESTAMPTZ) AS dt_due,
  NULLIF(dt_returned, '9999-01-01 00:00:00+00'::TIMESTAMPTZ) AS dt_returned
FROM library.loan
WHERE dt_returned = '9999-01-01 00:00:00+00'::TIMESTAMPTZ
  AND dt_due < CURRENT_TIMESTAMP
ORDER BY dt_due, loan_id;
