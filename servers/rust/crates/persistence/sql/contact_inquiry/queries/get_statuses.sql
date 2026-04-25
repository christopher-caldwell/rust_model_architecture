SELECT
      st.struct_type_id "id"
    , st.att_pub_ident  "status"
FROM
    brochure.struct_type st
WHERE
    st.group_name = 'contact_inquiry_status'
;
