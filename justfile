winittests:
    #cargo test --features winit_0_21 (doesn't work on arm macs)
    cargo test --features winit_0_24
    cargo test --features winit_0_27
    cargo test --features winit_0_29
    cargo test --features winit_0_30

test: winittests
    cargo test
    cargo fmt -- --check
    cargo clippy -- -D clippy::all

fmt:
    cargo fmt

