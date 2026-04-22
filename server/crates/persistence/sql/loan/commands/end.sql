WITH updated AS (
    UPDATE library.loan l
    SET
        dt_returned = CURRENT_TIMESTAMP
    WHERE
        l.loan_id = $1
    RETURNING
          l.loan_id
        , l.loan_ident
        , l.dt_created
        , l.dt_modified
        , l.book_copy_id
        , l.member_id
        , NULLIF(l.dt_due, '9999-01-01 00:00:00+00'::TIMESTAMPTZ) AS dt_due
        , NULLIF(l.dt_returned, '9999-01-01 00:00:00+00'::TIMESTAMPTZ) AS dt_returned
)
SELECT
      updated.loan_id
    , updated.loan_ident
    , updated.dt_created
    , updated.dt_modified
    , updated.book_copy_id
    , updated.member_id
    , updated.dt_due
    , updated.dt_returned
FROM
    updated
;
