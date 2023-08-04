# Build stage
FROM rust:latest AS build

# Create a new empty shell project
WORKDIR /usr/src/rusty
RUN USER=root cargo new --bin rusty

# Copy Cargo.toml and Cargo.lock to get dependencies cached
COPY Cargo.toml Cargo.lock ./rusty/

WORKDIR /usr/src/rusty/rusty

# Build dependencies
RUN cargo build --release

# Remove the dummy project, we will replace this with the real one next
RUN rm src/*.rs

# Now copy in our actual source code and build it
COPY src ./src
RUN cargo build --release

# Runtime stage
FROM alpine:latest AS runtime

# Install necessary runtime dependencies
RUN apk --no-cache add ca-certificates libgcc libstdc++

# Copy the binary from the build stage
COPY --from=build /usr/src/rusty/rusty/target/release/rusty /usr/local/bin

# Open the mapped port
EXPOSE 8080

# Command to run our app
CMD ["rusty"]
