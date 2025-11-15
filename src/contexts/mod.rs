use dioxus::prelude::*;

use crate::model::User;

#[derive(Clone)]
pub struct AuthUser {
    pub user: Signal<Option<User>>,
}

pub fn use_auth_provider() {
    use_context_provider(|| AuthUser {
        user: Signal::new(None),
    });
}

pub fn use_auth() -> AuthUser {
    use_context()
}
