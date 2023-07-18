use validators::prelude::*;

#[derive(Validator)]
#[validator(email(comment(NotAllow), ip(Allow), local(Allow), at_least_two_labels(Allow), non_ascii(Allow)))]
pub struct Email {
    pub local_part: String,
    pub need_quoted: bool,
    pub domain_part: validators::models::Host,
}


#[test]
fn is_should_create_an_email() {
    let email = Email::parse_string("joke@example.com");
    assert!(email.is_ok());
}