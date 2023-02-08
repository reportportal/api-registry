use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Because docs.rs uses an old version protobuf compiler that doesn't support
    // option fields for Proto3, we avoid executing the build script on docs.rs.
    if std::env::var("DOCS_RS").is_err() {
        let path = concat!("src/", env!("BUILD_DIR"));
        fs::create_dir_all(path)?;

        tonic_build::configure()
        .out_dir(path)
        .compile(
            &["api/proto/reportportal/reporting/v1/reporting.proto"],
            &["api/proto/"],
        )?;
    }
    
    Ok(())
}