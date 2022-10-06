FROM debian:bullseye
WORKDIR /service

# Update and Install tools and dependencies
RUN apt-get update -y
RUN apt-get upgrade -y --no-install-recommends
RUN apt-get install -y --no-install-recommends curl libssl-dev
# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build release files
RUN cargo build --release

  # Clean-up
  RUN rm -r ./src
  RUN mv ./target/release/TEMPLATE /usr/local/bin
  RUN chmod +x /usr/local/bin/TEMPLATE
  # Set CMD
  CMD ./target/release/TEMPLATE # REPLACE THIS