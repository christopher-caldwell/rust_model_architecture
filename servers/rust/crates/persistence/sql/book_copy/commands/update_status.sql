UPDATE library.book_copy bc
SET
    status_id = (
        SELECT st.struct_type_id
        FROM library.struct_type st
        WHERE st.group_name = 'book_copy_status'
          AND st.att_pub_ident = $2
    )
WHERE
    bc.book_copy_id = $1
;
