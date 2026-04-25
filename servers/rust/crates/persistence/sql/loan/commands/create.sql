WITH next_id AS (
    SELECT
        nextval(pg_get_serial_sequence('library.loan', 'loan_id'))::integer AS loan_id
), inserted AS (
    INSERT INTO library.loan (loan_id, loan_ident, book_copy_id, member_id)
    OVERRIDING SYSTEM VALUE
    SELECT
          next_id.loan_id
        , 'LN-' || lpad(next_id.loan_id::text, 6, '0')
        , $1
        , $2
    FROM
        next_id
    RETURNING
          loan_id
        , loan_ident
)
SELECT
      inserted.loan_id
    , inserted.loan_ident
FROM
    inserted
;
