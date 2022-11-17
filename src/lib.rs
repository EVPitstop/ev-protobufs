pub mod bookstore {
    include!("../generated/bookstore.rs");
    // pub const GRPC_HEALTH_V1_FILE_DESCRIPTOR_SET: &[u8] =
    //     include_bytes!("../generated/grpc_health_v1.bin");
}

pub mod greeter {
    include!("../generated/greeter.rs");
}
