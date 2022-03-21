///
/// This tells `tonic-build` to compile your protobufs when you build your
/// Rust project.
///
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n>>> Ready to compile proto files ......");

    // let _ = tonic_build::compile_protos("proto/echo.proto")?;

    let _ = tonic_build::configure()
        //
        // `tonic_build` generates the gRPC client and server
        // implementation by default. If you Do not generate
        // the gRPC server code, then you can enlabe this and
        // you need to write it by your own.
        //
        // .build_server(false)
        //
        //
        // After compiling, the proto implement Rust file will located in:
        //
        // `target/debug/build/grpc-demo-{random-id}/out/echo.rs`
        //
        .compile(&["proto/echo/echo.proto"], &["proto"])?;


    let out_dir = std::env::var("OUT_DIR").unwrap();
    println!(">>> Generated proto implementation into: {:?}", &out_dir);

    println!(">>> Compile proto files [ DONE ].\n");

    Ok(())
}
