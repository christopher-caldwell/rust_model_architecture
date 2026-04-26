/* @name CreateBook */
INSERT INTO library.book (isbn, title, author_name)
VALUES (:isbn!, :title!, :author_name!)
RETURNING book_id;

/* @name GetBookByIsbn */
SELECT book_id, isbn, dt_created, dt_modified, title, author_name
FROM library.book
WHERE isbn = :isbn!;

/* @name GetBookCatalog */
SELECT book_id, isbn, dt_created, dt_modified, title, author_name
FROM library.book
ORDER BY title, book_id;
