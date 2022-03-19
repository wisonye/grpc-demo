`Rust gRPC`

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

