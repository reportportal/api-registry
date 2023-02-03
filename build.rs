use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(&["api/proto/reportportal/reporting/v1/reporting.proto"], &["api/proto/"])?;
    Ok(())
}