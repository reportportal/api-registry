fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
    .compile(
        &["api/proto/reportportal/reporting/v1/reporting.proto"],
        &["api/proto/"],
    )?;
    
    Ok(())
}