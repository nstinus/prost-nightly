fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto/dummy.proto");
    tonic_build::compile_protos("proto/dummy.proto")?;

    Ok(())
}
