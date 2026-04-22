INSERT INTO library.member (member_ident, status_id, full_name, max_active_loans)
VALUES
(
      $1
    , (
          SELECT st.struct_type_id
          FROM library.struct_type st
          WHERE st.group_name = 'member_status'
            AND st.att_pub_ident = $2
      )
    , $3
    , $4
)
RETURNING
    member_id
;
