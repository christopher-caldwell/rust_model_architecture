BEGIN;

SET client_min_messages TO WARNING;
SET ROLE library_admin_role;
SET search_path TO library;


CREATE OR REPLACE FUNCTION set_dt_modified_column()
RETURNS TRIGGER AS
$$
    BEGIN
        NEW.dt_modified = NOW();
    RETURN NEW;
END;
$$
LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION update_dt_modified_column(tablename REGCLASS)
RETURNS VOID AS
$$
DECLARE
    schema_name TEXT;
    table_name TEXT;
    qualified_name TEXT;
BEGIN
    SELECT
          n.nspname
        , c.relname
    INTO STRICT
          schema_name
        , table_name
    FROM
        pg_catalog.pg_class c
    JOIN
        pg_catalog.pg_namespace n
    ON
        c.relnamespace = n.oid
    WHERE
        c.oid = tablename;

    qualified_name := quote_ident(schema_name) || '.' || quote_ident(table_name);

    EXECUTE FORMAT('DROP TRIGGER IF EXISTS set_dt_modified_column ON %s;', qualified_name);
    EXECUTE FORMAT('CREATE TRIGGER set_dt_modified_column BEFORE UPDATE ON %s FOR EACH ROW WHEN (OLD IS DISTINCT FROM NEW) EXECUTE FUNCTION set_dt_modified_column();', qualified_name);
END;
$$
LANGUAGE plpgsql;


DROP TABLE IF EXISTS struct_type CASCADE;
CREATE TABLE struct_type (
      struct_type_id                                            SMALLINT                                        NOT NULL GENERATED ALWAYS AS IDENTITY
    , dt_created                                                TIMESTAMPTZ                                     NOT NULL DEFAULT CURRENT_TIMESTAMP
    , dt_modified                                               TIMESTAMPTZ                                     NOT NULL DEFAULT CURRENT_TIMESTAMP
    , display_order                                             SMALLINT                                        NOT NULL
    , group_name                                                TEXT                                            NOT NULL
    , att_pub_ident                                             TEXT                                            NOT NULL
    , att_value                                                 TEXT                                            NOT NULL
    , PRIMARY KEY (struct_type_id)
)
;
SELECT update_dt_modified_column('struct_type');
CREATE UNIQUE INDEX idx_struct_type_group_ident                 ON struct_type                                  USING btree (group_name, att_pub_ident);


DROP TABLE IF EXISTS book CASCADE;
CREATE TABLE book (
      book_id                                                   INTEGER                                         NOT NULL GENERATED ALWAYS AS IDENTITY
    , dt_created                                                TIMESTAMPTZ                                     NOT NULL DEFAULT CURRENT_TIMESTAMP
    , dt_modified                                               TIMESTAMPTZ                                     NOT NULL DEFAULT CURRENT_TIMESTAMP
    , isbn                                                      TEXT                                            NOT NULL
    , title                                                     TEXT                                            NOT NULL
    , author_name                                               TEXT                                            NOT NULL
    , PRIMARY KEY (book_id)
)
;
CREATE UNIQUE INDEX idx_book_isbn                               ON book                                         USING btree (isbn);
SELECT update_dt_modified_column('book');


DROP TABLE IF EXISTS book_copy CASCADE;
CREATE TABLE book_copy (
      book_copy_id                                              INTEGER                                         NOT NULL GENERATED ALWAYS AS IDENTITY
    , dt_created                                                TIMESTAMPTZ                                     NOT NULL DEFAULT CURRENT_TIMESTAMP
    , dt_modified                                               TIMESTAMPTZ                                     NOT NULL DEFAULT CURRENT_TIMESTAMP
    , book_id                                                   INTEGER                                         NOT NULL
    , status_id                                                 SMALLINT                                        NOT NULL
    , barcode                                                   TEXT                                            NOT NULL
    , CONSTRAINT fk_book_copy_book_id                           FOREIGN KEY (book_id)                           REFERENCES book (book_id)
    , CONSTRAINT fk_book_copy_status_id                         FOREIGN KEY (status_id)                         REFERENCES struct_type (struct_type_id)
    , PRIMARY KEY (book_copy_id)
)
;
CREATE UNIQUE INDEX idx_book_copy_barcode                       ON book_copy                                    USING btree (barcode);
SELECT update_dt_modified_column('book_copy');


DROP TABLE IF EXISTS member CASCADE;
CREATE TABLE member (
      member_id                                                 INTEGER                                         NOT NULL GENERATED ALWAYS AS IDENTITY
    , member_ident                                              TEXT                                            NOT NULL
    , dt_created                                                TIMESTAMPTZ                                     NOT NULL DEFAULT CURRENT_TIMESTAMP
    , dt_modified                                               TIMESTAMPTZ                                     NOT NULL DEFAULT CURRENT_TIMESTAMP
    , status_id                                                 SMALLINT                                        NOT NULL
    , full_name                                                 TEXT                                            NOT NULL
    , max_active_loans                                          SMALLINT                                        NOT NULL DEFAULT 5
    , CONSTRAINT fk_member_status_id                            FOREIGN KEY (status_id)                         REFERENCES struct_type (struct_type_id)
    , PRIMARY KEY (member_id)
)
;
CREATE UNIQUE INDEX idx_member_member_ident                     ON member                                      USING btree (member_ident);
SELECT update_dt_modified_column('member');


DROP TABLE IF EXISTS loan CASCADE;
CREATE TABLE loan (
      loan_id                                                   INTEGER                                         NOT NULL GENERATED ALWAYS AS IDENTITY
    , loan_ident                                                TEXT                                            NOT NULL
    , dt_created                                                TIMESTAMPTZ                                     NOT NULL DEFAULT CURRENT_TIMESTAMP
    , dt_modified                                               TIMESTAMPTZ                                     NOT NULL DEFAULT CURRENT_TIMESTAMP
    , book_copy_id                                              INTEGER                                         NOT NULL
    , member_id                                                 INTEGER                                         NOT NULL
    , dt_due                                                    TIMESTAMPTZ                                     NOT NULL DEFAULT '9999-01-01'
    , dt_returned                                               TIMESTAMPTZ                                     NOT NULL DEFAULT '9999-01-01'
    , CONSTRAINT fk_loan_book_copy_id                           FOREIGN KEY (book_copy_id)                      REFERENCES book_copy (book_copy_id)
    , CONSTRAINT fk_loan_member_id                              FOREIGN KEY (member_id)                         REFERENCES member (member_id)
    , PRIMARY KEY (loan_id)
)
;
CREATE UNIQUE INDEX idx_loan_loan_ident                         ON loan                                         USING btree (loan_ident);
CREATE UNIQUE INDEX idx_loan_only_one_loan_per_copy             ON loan                                         USING btree (book_copy_id, dt_returned);
SELECT update_dt_modified_column('loan');


COMMIT;
