SELECT
      l.loan_id
    , l.loan_ident
    , l.dt_created
    , l.dt_modified
    , l.book_copy_id
    , l.member_id
    , NULLIF(l.dt_due, '9999-01-01 00:00:00+00'::TIMESTAMPTZ) AS dt_due
    , NULLIF(l.dt_returned, '9999-01-01 00:00:00+00'::TIMESTAMPTZ) AS dt_returned
FROM
    library.loan l
JOIN
    library.member m
ON
    l.member_id = m.member_id
WHERE
    m.member_ident = $1
ORDER BY
      l.dt_created DESC
    , l.loan_id DESC
;
