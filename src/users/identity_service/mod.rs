use crate::users::identity_repository::IdentityRepository;

pub struct IdentityService<'a> {
    identity_repository: IdentityRepository<'a>
}

