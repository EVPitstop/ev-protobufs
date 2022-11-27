pub mod user {
    include!("../generated/user.v1.rs");
    // pub const GRPC_HEALTH_V1_FILE_DESCRIPTOR_SET: &[u8] =
    //     include_bytes!("../generated/grpc_health_v1.bin");
}

pub mod health {
    include!("../generated/health.v1.rs");
}
