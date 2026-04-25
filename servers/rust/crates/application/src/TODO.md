Yes.

## Membership commands

Keep membership very small.

### `RegisterMember`

Creates a new member.

Input:

- name
- maybe email or member ident
- maybe max active loans if not defaulted

Effect:

- inserts member
- initial status is usually `active`

---

### `SuspendMember`

Prevents future borrowing.

Input:

- member id

Effect:

- member status becomes `suspended`

Rule:

- suspended members cannot check out new copies

---

### `ReactivateMember`

Moves a suspended member back to active.

Input:

- member id

Effect:

- member status becomes `active`

I would include this for symmetry.

---

## Lending commands

This is the borrow/return lifecycle.

### `CheckOutBookCopy`

The main lending command.

Input:

- member id
- book copy id
- maybe due date, or calculate it inside

Effect:

- verifies member can borrow
- verifies copy can be borrowed
- creates loan

If you are deriving `on_loan` from active loans, this command does **not** update copy status to `on_loan`.

---

### `ReturnBookCopy`

Closes an active loan.

Input:

- book copy id
- maybe member id too, but usually not necessary

Effect:

- finds active loan for the copy
- sets returned timestamp

---

### `ReportLostLoanedBookCopy`

This is the cross-boundary one we discussed.

Input:

- book copy id
- maybe member id, depending on how explicit you want to be

Effect:

- finds active loan
- handles the lending-side lost flow
- marks the copy itself as lost too

This belongs in lending because the initiating event is a lost borrowed item.

---

## All queries

I would keep queries grouped by capability in code, but here’s the full set.

## Catalog queries

### `GetBookCatalog`

Returns the library catalog view.

Likely fields:

- book id
- title
- author
- isbn
- total copies
- available copies

---

### `GetBookCopyDetails`

Returns one copy and its current state.

Likely fields:

- book copy id
- barcode
- title
- isbn
- physical status
- whether currently on loan

---

## Membership queries

### `GetMemberDetails`

Returns one member.

Likely fields:

- member id
- name
- status
- max active loans
- current active loan count

---

### `GetMembers`

Simple member listing.

Likely fields:

- member id
- name
- status

This one is optional if you want to stay very small.

---

## Lending queries

### `GetMemberLoans`

Returns the loans for a member.

Likely fields:

- loan id
- book title
- barcode
- loaned at
- due at
- returned at
- maybe derived status like active/returned/overdue

This is one of the best read-side demos.

---

### `GetActiveLoans`

Returns all currently active loans.

Likely fields:

- member name
- title
- barcode
- due date

Optional, but useful.

---

### `GetOverdueLoans`

Returns active loans where due date has passed.

Likely fields:

- member
- title
- barcode
- due date
- days overdue

This is another very strong CQRS-style query.

---

## My recommended minimal set

If you want the cleanest demo scope:

### Commands

**Catalog**

- `AddBook`
- `AddBookCopy`
- `MarkBookCopyLost`
- `MarkBookCopyFound`
- `SendBookCopyToMaintenance`
- `CompleteBookCopyMaintenance`

**Membership**

- `RegisterMember`
- `SuspendMember`
- `ReactivateMember`

**Lending**

- `CheckOutBookCopy`
- `ReturnBookCopy`
- `ReportLostLoanedBookCopy`

### Queries

**Catalog**

- `GetBookCatalog`
- `GetBookCopyDetails`

**Membership**

- `GetMemberDetails`

**Lending**

- `GetMemberLoans`
- `GetOverdueLoans`

That is probably the sweet spot.

Next I’d lock which of these return full domain models vs read DTOs.

---

## Implementation Checklist

### Commands

#### Catalog

- [x] `AddBook`
- [x] `AddBookCopy`
- [x] `MarkBookCopyLost`
- [x] `MarkBookCopyFound`
- [x] `SendBookCopyToMaintenance`
- [x] `CompleteBookCopyMaintenance`

#### Membership

- [x] `RegisterMember`
- [x] `SuspendMember`
- [x] `ReactivateMember`

#### Lending

- [ ] `CheckOutBookCopy`
- [ ] `ReturnBookCopy`
- [ ] `ReportLostLoanedBookCopy`

### Queries

#### Catalog

- [x] `GetBookCatalog`
- [x] `GetBookCopyDetails`

#### Membership

- [x] `GetMemberDetails`

#### Lending

- [ ] `GetMemberLoans`
- [ ] `GetOverdueLoans`
