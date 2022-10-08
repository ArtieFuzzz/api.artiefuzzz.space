FROM rust:slim
WORKDIR /service

SHELL ["/bin/bash", "-c"]

# Update and Install tools and dependencies
RUN apt-get update -y
RUN apt-get upgrade -y --no-install-recommends
RUN apt-get install -y --no-install-recommends libssl-dev ca-certificates build-essential gcc pkg-config

COPY . .

# Build release files
RUN cargo build --release

# Clean-up
RUN rm -r ./src
RUN mv ./target/release/api /usr/local/bin
RUN chmod +x /usr/local/bin/api
# Set CMD
CMD api

ENV BIND_PORT 8080
ENV BIND_ADDRESS 0.0.0.0
EXPOSE 8080