WITH updated AS (
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
    RETURNING
          bc.book_copy_id
        , bc.barcode
        , bc.dt_created
        , bc.dt_modified
        , bc.book_id
        , bc.status_id
)
SELECT
      updated.book_copy_id
    , updated.barcode
    , updated.dt_created
    , updated.dt_modified
    , updated.book_id
    , b.author_name
    , st.att_pub_ident "status"
FROM
    updated
JOIN
    library.book b
ON
    updated.book_id = b.book_id
JOIN
    library.struct_type st
ON
    updated.status_id = st.struct_type_id
;
