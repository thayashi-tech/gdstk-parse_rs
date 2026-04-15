test-build: 
    cargo build --example oasis_to_image --release
    cargo build --example cutout --release

test-init-data:
    cd test && sh make.sh

test: test-build
    cd test && uv run python run.py

test-all: test
