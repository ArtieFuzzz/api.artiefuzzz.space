FROM debian:bullseye
WORKDIR /service

# Update and Install tools and dependencies
RUN apt-get update -y
RUN apt-get upgrade -y --no-install-recommends
RUN apt-get install -y --no-install-recommends curl libssl-dev ca-certificates
# Install Rust
RUN curl https://sh.rustup.rs -sSf | sh

# Build release files
RUN cargo build --release

  # Clean-up
  RUN rm -r ./src
  RUN mv ./target/release/api /usr/local/bin
  RUN chmod +x /usr/local/bin/api
  # Set CMD
  CMD api