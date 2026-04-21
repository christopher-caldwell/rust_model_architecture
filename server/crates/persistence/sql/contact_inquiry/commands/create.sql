INSERT INTO brochure.contact_inquiry (contact_inquiry_ident, status_id, first_name, last_name, email, phone_number, source, website_given, message, spam_likelihood)
VALUES
(
      $1
    , (SELECT struct_type_id FROM brochure.struct_type st WHERE st.att_pub_ident = $2 AND st.group_name = 'contact_inquiry_status')
    , $3
    , $4
    , $5
    , $6
    , $7
    , $8
    , $9
    , $10
)
RETURNING
    contact_inquiry_id
;
