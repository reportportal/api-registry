pub mod reportportal_grpc {
    pub mod reporting {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/reportportal.reporting.v1.rs"));
        }
    }
}

pub mod google {
    pub mod r#type {
        include!(concat!(env!("OUT_DIR"), "/google.r#type.rs"));
    }
}