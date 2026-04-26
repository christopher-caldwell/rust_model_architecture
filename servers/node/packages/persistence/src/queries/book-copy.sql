/* @name CreateBookCopy */
INSERT INTO library.book_copy (book_id, status_id, barcode)
VALUES (
  :book_id!,
  (
    SELECT struct_type_id
    FROM library.struct_type
    WHERE group_name = 'book_copy_status'
      AND att_pub_ident = :status!
  ),
  :barcode!
)
RETURNING book_copy_id;

/* @name GetBookCopyById */
SELECT bc.book_copy_id, bc.barcode, bc.dt_created, bc.dt_modified, bc.book_id, st.att_pub_ident AS status
FROM library.book_copy bc
JOIN library.struct_type st ON bc.status_id = st.struct_type_id
WHERE bc.book_copy_id = :book_copy_id!;

/* @name GetBookCopyByBarcode */
SELECT bc.book_copy_id, bc.barcode, bc.dt_created, bc.dt_modified, bc.book_id, st.att_pub_ident AS status
FROM library.book_copy bc
JOIN library.struct_type st ON bc.status_id = st.struct_type_id
WHERE bc.barcode = :barcode!;

/* @name GetBookCopyByBarcodeForUpdate */
SELECT bc.book_copy_id, bc.barcode, bc.dt_created, bc.dt_modified, bc.book_id, st.att_pub_ident AS status
FROM library.book_copy bc
JOIN library.struct_type st ON bc.status_id = st.struct_type_id
WHERE bc.barcode = :barcode!
FOR UPDATE OF bc;

/* @name UpdateBookCopyStatus */
UPDATE library.book_copy
SET status_id = (
  SELECT struct_type_id
  FROM library.struct_type
  WHERE group_name = 'book_copy_status'
    AND att_pub_ident = :status!
)
WHERE book_copy_id = :book_copy_id!;
