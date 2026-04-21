use std::fs::{self, File};
use std::io::{BufWriter, Write};

const OUT: &str = "output";

// ── domain types ────────────────────────────────────────────────────────────

#[derive(Debug)]
struct StructType {
    struct_type_id: i16,
    display_order: i16,
    group_name: &'static str,
    att_pub_ident: &'static str,
    att_value: &'static str,
}

#[derive(Debug)]
struct Book {
    book_id: i32,
    isbn: &'static str,
    title: &'static str,
    author_name: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CopyStatus {
    Active = 1,
    Lost = 2,
    Maintenance = 3,
}

#[derive(Debug)]
struct BookCopy {
    book_copy_id: i32,
    book_id: i32,
    status: CopyStatus,
    barcode: String,
}

#[derive(Debug, Clone, Copy)]
enum MemberStatus {
    Active = 4,
    Suspended = 5,
    Removed = 6,
}

#[derive(Debug)]
struct Member {
    member_id: i32,
    member_ident: &'static str,
    status: MemberStatus,
    full_name: &'static str,
    max_active_loans: i16,
}

#[derive(Debug)]
struct Loan {
    loan_id: i32,
    loan_ident: &'static str,
    book_copy_id: i32,
    member_id: i32,
    dt_due: &'static str,
    dt_returned: &'static str,
}

// ── tsv serialization ────────────────────────────────────────────────────────

impl StructType {
    fn to_tsv(&self) -> String {
        format!(
            "{}\t{}\t{}\t{}\t{}",
            self.struct_type_id,
            self.display_order,
            self.group_name,
            self.att_pub_ident,
            self.att_value,
        )
    }
}

impl Book {
    fn to_tsv(&self) -> String {
        format!(
            "{}\t{}\t{}\t{}",
            self.book_id, self.isbn, self.title, self.author_name,
        )
    }
}

impl BookCopy {
    fn to_tsv(&self) -> String {
        format!(
            "{}\t{}\t{}\t{}",
            self.book_copy_id,
            self.book_id,
            self.status as i16,
            self.barcode,
        )
    }
}

impl Member {
    fn to_tsv(&self) -> String {
        format!(
            "{}\t{}\t{}\t{}\t{}",
            self.member_id,
            self.member_ident,
            self.status as i16,
            self.full_name,
            self.max_active_loans,
        )
    }
}

impl Loan {
    fn to_tsv(&self) -> String {
        format!(
            "{}\t{}\t{}\t{}\t{}\t{}",
            self.loan_id,
            self.loan_ident,
            self.book_copy_id,
            self.member_id,
            self.dt_due,
            self.dt_returned,
        )
    }
}

// ── seed data ────────────────────────────────────────────────────────────────

fn struct_types() -> Vec<StructType> {
    vec![
        StructType { struct_type_id: 1, display_order: 1, group_name: "book_copy_status", att_pub_ident: "active",      att_value: "Active"      },
        StructType { struct_type_id: 2, display_order: 2, group_name: "book_copy_status", att_pub_ident: "lost",         att_value: "Lost"        },
        StructType { struct_type_id: 3, display_order: 3, group_name: "book_copy_status", att_pub_ident: "maintenance",  att_value: "Maintenance" },
        StructType { struct_type_id: 4, display_order: 1, group_name: "member_status",    att_pub_ident: "active",       att_value: "Active"      },
        StructType { struct_type_id: 5, display_order: 2, group_name: "member_status",    att_pub_ident: "suspended",    att_value: "Suspended"   },
        StructType { struct_type_id: 6, display_order: 3, group_name: "member_status",    att_pub_ident: "removed",      att_value: "Removed"     },
    ]
}

fn books() -> Vec<Book> {
    vec![
        Book { book_id: 1,  isbn: "9780135957059", title: "The Pragmatic Programmer",                         author_name: "David Thomas, Andrew Hunt"        },
        Book { book_id: 2,  isbn: "9780132350884", title: "Clean Code",                                        author_name: "Robert C. Martin"                 },
        Book { book_id: 3,  isbn: "9780201633610", title: "Design Patterns",                                   author_name: "Erich Gamma et al."               },
        Book { book_id: 4,  isbn: "9780201835953", title: "The Mythical Man-Month",                            author_name: "Fred Brooks"                      },
        Book { book_id: 5,  isbn: "9780262510875", title: "Structure and Interpretation of Computer Programs", author_name: "Harold Abelson, Gerald Sussman"    },
        Book { book_id: 6,  isbn: "9780262033848", title: "Introduction to Algorithms",                        author_name: "Thomas Cormen et al."             },
        Book { book_id: 7,  isbn: "9780201896831", title: "The Art of Computer Programming Vol. 1",            author_name: "Donald Knuth"                     },
        Book { book_id: 8,  isbn: "9780134757599", title: "Refactoring",                                       author_name: "Martin Fowler"                    },
        Book { book_id: 9,  isbn: "9780131177055", title: "Working Effectively with Legacy Code",              author_name: "Michael Feathers"                 },
        Book { book_id: 10, isbn: "9780321125217", title: "Domain-Driven Design",                              author_name: "Eric Evans"                       },
        Book { book_id: 11, isbn: "9780321127426", title: "Patterns of Enterprise Application Architecture",   author_name: "Martin Fowler"                    },
        Book { book_id: 12, isbn: "9780321146533", title: "Test-Driven Development By Example",                author_name: "Kent Beck"                        },
        Book { book_id: 13, isbn: "9780137081073", title: "The Clean Coder",                                   author_name: "Robert C. Martin"                 },
        Book { book_id: 14, isbn: "9780321601919", title: "Continuous Delivery",                               author_name: "Jez Humble, David Farley"         },
        Book { book_id: 15, isbn: "9781942788294", title: "The Phoenix Project",                               author_name: "Gene Kim et al."                  },
        Book { book_id: 16, isbn: "9781491929124", title: "Site Reliability Engineering",                      author_name: "Betsy Beyer et al."               },
        Book { book_id: 17, isbn: "9781617293726", title: "Kubernetes in Action",                              author_name: "Marko Luksa"                      },
        Book { book_id: 18, isbn: "9781492040347", title: "Database Internals",                                author_name: "Alex Petrov"                      },
        Book { book_id: 19, isbn: "9781449373320", title: "Designing Data-Intensive Applications",             author_name: "Martin Kleppmann"                 },
        Book { book_id: 20, isbn: "9781593273897", title: "The Linux Command Line",                            author_name: "William Shotts"                   },
        Book { book_id: 21, isbn: "9781449355739", title: "Learning Python",                                   author_name: "Mark Lutz"                        },
        Book { book_id: 22, isbn: "9781492056355", title: "Fluent Python",                                     author_name: "Luciano Ramalho"                  },
        Book { book_id: 23, isbn: "9781718500440", title: "The Rust Programming Language",                     author_name: "Steve Klabnik, Carol Nichols"      },
        Book { book_id: 24, isbn: "9781492052555", title: "Programming Rust",                                  author_name: "Jim Blandy et al."                },
        Book { book_id: 25, isbn: "9780134190440", title: "The Go Programming Language",                       author_name: "Alan Donovan, Brian Kernighan"     },
        Book { book_id: 26, isbn: "9780134685991", title: "Effective Java",                                    author_name: "Joshua Bloch"                     },
        Book { book_id: 27, isbn: "9780321349606", title: "Java Concurrency in Practice",                      author_name: "Brian Goetz et al."               },
        Book { book_id: 28, isbn: "9780596517748", title: "JavaScript: The Good Parts",                        author_name: "Douglas Crockford"                },
        Book { book_id: 29, isbn: "9781491924464", title: "You Don't Know JS",                                  author_name: "Kyle Simpson"                     },
        Book { book_id: 30, isbn: "9781839214110", title: "Node.js Design Patterns",                           author_name: "Mario Casciaro"                   },
    ]
}

fn book_copies() -> Vec<BookCopy> {
    // 5 copies per book (150 total). ~14% non-active via copy_id % 7 == 0.
    //   odd multiple of 7  → Lost
    //   even multiple of 7 → Maintenance
    (1i32..=150)
        .map(|copy_id| {
            let book_id = (copy_id - 1) / 5 + 1;
            let status = if copy_id % 7 == 0 {
                if copy_id % 2 == 1 { CopyStatus::Lost } else { CopyStatus::Maintenance }
            } else {
                CopyStatus::Active
            };
            BookCopy {
                book_copy_id: copy_id,
                book_id,
                status,
                barcode: format!("BC-{copy_id:05}"),
            }
        })
        .collect()
}

fn members() -> Vec<Member> {
    vec![
        Member { member_id: 1,  member_ident: "MBR-001", status: MemberStatus::Active,    full_name: "Alice Johnson",  max_active_loans: 5 },
        Member { member_id: 2,  member_ident: "MBR-002", status: MemberStatus::Active,    full_name: "Bob Smith",      max_active_loans: 5 },
        Member { member_id: 3,  member_ident: "MBR-003", status: MemberStatus::Active,    full_name: "Carol Williams", max_active_loans: 5 },
        Member { member_id: 4,  member_ident: "MBR-004", status: MemberStatus::Active,    full_name: "David Brown",    max_active_loans: 5 },
        Member { member_id: 5,  member_ident: "MBR-005", status: MemberStatus::Active,    full_name: "Eve Davis",      max_active_loans: 5 },
        Member { member_id: 6,  member_ident: "MBR-006", status: MemberStatus::Active,    full_name: "Frank Miller",   max_active_loans: 5 },
        Member { member_id: 7,  member_ident: "MBR-007", status: MemberStatus::Active,    full_name: "Grace Wilson",   max_active_loans: 5 },
        Member { member_id: 8,  member_ident: "MBR-008", status: MemberStatus::Active,    full_name: "Henry Moore",    max_active_loans: 5 },
        Member { member_id: 9,  member_ident: "MBR-009", status: MemberStatus::Active,    full_name: "Iris Taylor",    max_active_loans: 5 },
        Member { member_id: 10, member_ident: "MBR-010", status: MemberStatus::Active,    full_name: "Jack Anderson",  max_active_loans: 5 },
        Member { member_id: 11, member_ident: "MBR-011", status: MemberStatus::Active,    full_name: "Karen Thomas",   max_active_loans: 5 },
        Member { member_id: 12, member_ident: "MBR-012", status: MemberStatus::Active,    full_name: "Liam Jackson",   max_active_loans: 5 },
        Member { member_id: 13, member_ident: "MBR-013", status: MemberStatus::Active,    full_name: "Mia White",      max_active_loans: 5 },
        Member { member_id: 14, member_ident: "MBR-014", status: MemberStatus::Active,    full_name: "Noah Harris",    max_active_loans: 5 },
        Member { member_id: 15, member_ident: "MBR-015", status: MemberStatus::Suspended, full_name: "Oscar Martinez", max_active_loans: 5 },
        Member { member_id: 16, member_ident: "MBR-016", status: MemberStatus::Suspended, full_name: "Paula Robinson", max_active_loans: 5 },
        Member { member_id: 17, member_ident: "MBR-017", status: MemberStatus::Suspended, full_name: "Quinn Clark",    max_active_loans: 5 },
        Member { member_id: 18, member_ident: "MBR-018", status: MemberStatus::Removed,   full_name: "Rachel Lewis",   max_active_loans: 5 },
    ]
}

const UNRETURNED: &str = "9999-01-01 00:00:00+00";

fn loans() -> Vec<Loan> {
    // 14 of 18 members have loans (~78%). Liam (12) and Noah (14) have none.
    // All book_copy_ids used here are active (none are multiples of 7).
    vec![
        // Alice (1): 2 active
        Loan { loan_id: 1,  loan_ident: "LN-001", book_copy_id: 1,   member_id: 1,  dt_due: "2026-05-01 00:00:00+00", dt_returned: UNRETURNED },
        Loan { loan_id: 2,  loan_ident: "LN-002", book_copy_id: 6,   member_id: 1,  dt_due: "2026-05-15 00:00:00+00", dt_returned: UNRETURNED },

        // Bob (2): 1 active
        Loan { loan_id: 3,  loan_ident: "LN-003", book_copy_id: 11,  member_id: 2,  dt_due: "2026-04-30 00:00:00+00", dt_returned: UNRETURNED },

        // Carol (3): 1 returned, 1 active
        Loan { loan_id: 4,  loan_ident: "LN-004", book_copy_id: 16,  member_id: 3,  dt_due: "2026-02-28 00:00:00+00", dt_returned: "2026-02-20 00:00:00+00" },
        Loan { loan_id: 5,  loan_ident: "LN-005", book_copy_id: 20,  member_id: 3,  dt_due: "2026-05-10 00:00:00+00", dt_returned: UNRETURNED },

        // David (4): 1 overdue active
        Loan { loan_id: 6,  loan_ident: "LN-006", book_copy_id: 25,  member_id: 4,  dt_due: "2026-03-15 00:00:00+00", dt_returned: UNRETURNED },

        // Eve (5): 1 active
        Loan { loan_id: 7,  loan_ident: "LN-007", book_copy_id: 31,  member_id: 5,  dt_due: "2026-05-20 00:00:00+00", dt_returned: UNRETURNED },

        // Frank (6): 2 returned, 1 active
        Loan { loan_id: 8,  loan_ident: "LN-008", book_copy_id: 36,  member_id: 6,  dt_due: "2026-01-15 00:00:00+00", dt_returned: "2026-01-10 00:00:00+00" },
        Loan { loan_id: 9,  loan_ident: "LN-009", book_copy_id: 40,  member_id: 6,  dt_due: "2026-02-15 00:00:00+00", dt_returned: "2026-02-12 00:00:00+00" },
        Loan { loan_id: 10, loan_ident: "LN-010", book_copy_id: 45,  member_id: 6,  dt_due: "2026-06-01 00:00:00+00", dt_returned: UNRETURNED },

        // Grace (7): 1 active
        Loan { loan_id: 11, loan_ident: "LN-011", book_copy_id: 50,  member_id: 7,  dt_due: "2026-05-25 00:00:00+00", dt_returned: UNRETURNED },

        // Henry (8): 1 returned
        Loan { loan_id: 12, loan_ident: "LN-012", book_copy_id: 55,  member_id: 8,  dt_due: "2026-03-01 00:00:00+00", dt_returned: "2026-02-25 00:00:00+00" },

        // Iris (9): 2 active
        Loan { loan_id: 13, loan_ident: "LN-013", book_copy_id: 60,  member_id: 9,  dt_due: "2026-05-05 00:00:00+00", dt_returned: UNRETURNED },
        Loan { loan_id: 14, loan_ident: "LN-014", book_copy_id: 65,  member_id: 9,  dt_due: "2026-05-08 00:00:00+00", dt_returned: UNRETURNED },

        // Jack (10): 1 overdue active
        Loan { loan_id: 15, loan_ident: "LN-015", book_copy_id: 66,  member_id: 10, dt_due: "2026-03-31 00:00:00+00", dt_returned: UNRETURNED },

        // Karen (11): 1 active
        Loan { loan_id: 16, loan_ident: "LN-016", book_copy_id: 71,  member_id: 11, dt_due: "2026-06-10 00:00:00+00", dt_returned: UNRETURNED },

        // Mia (13): 1 active
        Loan { loan_id: 17, loan_ident: "LN-017", book_copy_id: 80,  member_id: 13, dt_due: "2026-05-30 00:00:00+00", dt_returned: UNRETURNED },

        // Oscar (15, suspended): 2 overdue — reason for suspension
        Loan { loan_id: 18, loan_ident: "LN-018", book_copy_id: 85,  member_id: 15, dt_due: "2026-02-01 00:00:00+00", dt_returned: UNRETURNED },
        Loan { loan_id: 19, loan_ident: "LN-019", book_copy_id: 90,  member_id: 15, dt_due: "2026-02-15 00:00:00+00", dt_returned: UNRETURNED },

        // Paula (16, suspended): 1 overdue
        Loan { loan_id: 20, loan_ident: "LN-020", book_copy_id: 95,  member_id: 16, dt_due: "2026-01-20 00:00:00+00", dt_returned: UNRETURNED },

        // Quinn (17, suspended): 1 overdue
        Loan { loan_id: 21, loan_ident: "LN-021", book_copy_id: 100, member_id: 17, dt_due: "2026-03-10 00:00:00+00", dt_returned: UNRETURNED },

        // Rachel (18, removed): 1 returned historical loan
        Loan { loan_id: 22, loan_ident: "LN-022", book_copy_id: 101, member_id: 18, dt_due: "2025-12-15 00:00:00+00", dt_returned: "2025-12-10 00:00:00+00" },
    ]
}

// ── main ─────────────────────────────────────────────────────────────────────

fn write_tsv<T, F: Fn(&T) -> String>(table: &str, rows: &[T], serialize: F) {
    let mut w = BufWriter::new(File::create(format!("{OUT}/{table}.tsv")).unwrap());
    for row in rows {
        writeln!(w, "{}", serialize(row)).unwrap();
    }
}

fn main() {
    fs::create_dir_all(OUT).unwrap();

    write_tsv("struct_type", &struct_types(), |r| r.to_tsv());
    write_tsv("book",        &books(),        |r| r.to_tsv());
    write_tsv("book_copy",   &book_copies(),  |r| r.to_tsv());
    write_tsv("member",      &members(),      |r| r.to_tsv());
    write_tsv("loan",        &loans(),        |r| r.to_tsv());

    eprintln!("Done — TSV files written to {OUT}/");
}
