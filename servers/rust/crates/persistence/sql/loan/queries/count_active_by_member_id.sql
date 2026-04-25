SELECT
    COUNT(*)::BIGINT AS "count!"
FROM
    library.loan l
WHERE
    l.member_id = $1
AND
    l.dt_returned = '9999-01-01 00:00:00+00'::TIMESTAMPTZ
;
