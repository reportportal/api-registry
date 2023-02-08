pub mod reportportal_grpc {
    pub mod reporting {
        pub mod v1 {
            include!("build/proto/rust/reportportal.reporting.v1.rs");
        }
    }
}

pub mod google {
    pub mod r#type {
        include!("build/proto/rust/google.r#type.rs");
    }
}