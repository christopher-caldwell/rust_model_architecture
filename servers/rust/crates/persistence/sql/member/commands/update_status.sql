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
;
