/* @name CreateMember */
INSERT INTO library.member (member_ident, status_id, full_name, max_active_loans)
VALUES (
  :member_ident!,
  (
    SELECT struct_type_id
    FROM library.struct_type
    WHERE group_name = 'member_status'
      AND att_pub_ident = :status!
  ),
  :full_name!,
  :max_active_loans!
)
RETURNING member_id;

/* @name GetMemberById */
SELECT m.member_id, m.member_ident, m.dt_created, m.dt_modified, st.att_pub_ident AS status, m.full_name, m.max_active_loans
FROM library.member m
JOIN library.struct_type st ON m.status_id = st.struct_type_id
WHERE m.member_id = :member_id!
  AND st.group_name = 'member_status'
  AND st.att_pub_ident IN ('active', 'suspended');

/* @name GetMemberByIdent */
SELECT m.member_id, m.member_ident, m.dt_created, m.dt_modified, st.att_pub_ident AS status, m.full_name, m.max_active_loans
FROM library.member m
JOIN library.struct_type st ON m.status_id = st.struct_type_id
WHERE m.member_ident = :member_ident!
  AND st.group_name = 'member_status'
  AND st.att_pub_ident IN ('active', 'suspended');

/* @name GetMemberByIdentForUpdate */
SELECT m.member_id, m.member_ident, m.dt_created, m.dt_modified, st.att_pub_ident AS status, m.full_name, m.max_active_loans
FROM library.member m
JOIN library.struct_type st ON m.status_id = st.struct_type_id
WHERE m.member_ident = :member_ident!
  AND st.group_name = 'member_status'
  AND st.att_pub_ident IN ('active', 'suspended')
FOR UPDATE OF m;

/* @name UpdateMemberStatus */
UPDATE library.member
SET status_id = (
  SELECT struct_type_id
  FROM library.struct_type
  WHERE group_name = 'member_status'
    AND att_pub_ident = :status!
)
WHERE member_id = :member_id!;
