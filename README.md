`Rust gRPC`

- Related docs, repo and tooling:

    - [`Introduction to gRPC`](https://grpc.io/docs/what-is-grpc/introduction/)

    - [`Core concepts, architecture and lifecycle`](https://grpc.io/docs/what-is-grpc/core-concepts/)

    - [`Google Protocl Buffer`](https://developers.google.com/protocol-buffers/docs/overview)

    - [`prost`](https://crates.io/crates/prost): `prost` is a Protocol Buffers
    implementation for the Rust Language.

    - [`tonic`](https://github.com/hyperium/tonic): A rust implementation of
    gRPC, a high performance, open source, general RPC framework that puts
    mobile and HTTP/2 first.

    - [`bloomrpc`](https://github.com/bloomrpc/bloomrpc): gPRC testing GUI.

    - [`grpcurl`](https://github.com/fullstorydev/grpcurl): gRPC testing CLI.

    </br>


- Echo service

    - How to run gRPC server

        ```bash
        cargo watch -c --exec "run --bin echo"
        ```

    - How to test

        ```bash
        grpcurl -plaintext \
            -import-path ./proto \
            -proto echo/echo.proto \
            -d '{"content": "Hey buddy:)"}' \
            127.0.0.1:6000 \
            echo.Echo/saySomething
        ```

        </br>

