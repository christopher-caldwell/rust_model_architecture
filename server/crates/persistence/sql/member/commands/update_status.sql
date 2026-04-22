WITH updated AS (
    UPDATE library.member m
    SET
        status_id = (
            SELECT st.struct_type_id
            FROM library.struct_type st
            WHERE st.group_name = 'member_status'
              AND st.att_pub_ident = $2
        )
    WHERE
        m.member_id = $1
    RETURNING
          m.member_id
        , m.member_ident
        , m.dt_created
        , m.dt_modified
        , m.full_name
        , m.max_active_loans
        , m.status_id
)
SELECT
      updated.member_id
    , updated.member_ident
    , updated.dt_created
    , updated.dt_modified
    , st.att_pub_ident "status"
    , updated.full_name
    , updated.max_active_loans
FROM
    updated
JOIN
    library.struct_type st
ON
    updated.status_id = st.struct_type_id
;
