INSERT INTO library.book (isbn, title, author_name)
VALUES
(
      $1
    , $2
    , $3
)
RETURNING
    book_id
;
