#[macro_use]
extern crate serde;

pub mod user {
    include!("../generated/user.v1.rs");
}

pub mod health {
    include!("../generated/health.v1.rs");
}
