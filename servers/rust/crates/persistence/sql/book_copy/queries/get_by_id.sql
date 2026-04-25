SELECT
      bc.book_copy_id
    , bc.barcode
    , bc.dt_created
    , bc.dt_modified
    , bc.book_id
    , b.author_name
    , st.att_pub_ident "status"
FROM
    library.book_copy bc
JOIN
    library.book b
ON
    bc.book_id = b.book_id
JOIN
    library.struct_type st
ON
    bc.status_id = st.struct_type_id
WHERE
    bc.book_copy_id = $1
;
