SELECT
      ci.contact_inquiry_id
    , ci.contact_inquiry_ident
    , ci.dt_created
    , ci.dt_modified
    , st.att_pub_ident          "status"
    , ci.first_name
    , ci.last_name
    , ci.email
    , ci.phone_number
    , ci.source
    , ci.website_given
    , ci.message
    , ci.spam_likelihood
FROM
    brochure.contact_inquiry ci
JOIN
    brochure.struct_type st
ON
    ci.status_id = st.struct_type_id
WHERE
    contact_inquiry_id = $1
;
