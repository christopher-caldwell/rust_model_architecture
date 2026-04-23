SELECT
      m.member_id
    , m.member_ident
    , m.dt_created
    , m.dt_modified
    , st.att_pub_ident "status"
    , m.full_name
    , m.max_active_loans
FROM
    library.member m
JOIN
    library.struct_type st
ON
    m.status_id = st.struct_type_id
WHERE
    m.member_ident = $1
AND
    st.group_name = 'member_status'
AND
    st.att_pub_ident IN ('active', 'suspended')
FOR UPDATE OF m
;
