UPDATE library.loan l
SET
    dt_returned = CURRENT_TIMESTAMP
WHERE
    l.loan_id = $1
;
