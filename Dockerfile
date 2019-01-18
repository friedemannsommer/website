# -*- mode: dockerfile -*-
FROM ekidd/rust-musl-builder AS builder
ADD . ./
# Fix permissions on source code.
RUN sudo chown -R rust:rust /home/rust
# Create release
CMD cargo build --release
