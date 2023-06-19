fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("src/lib/encode_decode/proto/user_authorization.proto")?;
    Ok(())
}
