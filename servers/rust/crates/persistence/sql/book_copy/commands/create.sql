INSERT INTO library.book_copy (book_id, status_id, barcode)
VALUES
(
      $1
    , (
          SELECT st.struct_type_id
          FROM library.struct_type st
          WHERE st.group_name = 'book_copy_status'
            AND st.att_pub_ident = $2
      )
    , $3
)
RETURNING
    book_copy_id
;
