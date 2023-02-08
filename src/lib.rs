pub mod reportportal_grpc {
    pub mod reporting {
        pub mod v1 {
            include!(concat!(env!("BUILD_DIR"), "/reportportal.reporting.v1.rs"));
        }
    }
}

pub mod google {
    pub mod r#type {
        include!(concat!(env!("BUILD_DIR"), "/google.r#type.rs"));
    }
}