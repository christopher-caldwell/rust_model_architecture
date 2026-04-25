SELECT
      b.book_id
    , b.isbn
    , b.dt_created
    , b.dt_modified
    , b.title
    , b.author_name
FROM
    library.book b
WHERE
    b.isbn = $1
;
