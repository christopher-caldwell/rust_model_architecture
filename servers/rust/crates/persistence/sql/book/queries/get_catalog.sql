SELECT
      b.book_id
    , b.isbn
    , b.dt_created
    , b.dt_modified
    , b.title
    , b.author_name
FROM
    library.book b
ORDER BY
      b.title
    , b.book_id
;
