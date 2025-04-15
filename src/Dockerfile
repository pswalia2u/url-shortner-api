# Stage 1: Build
ARG PACKAGE=myapp
FROM cgr.dev/chainguard/rust as build
WORKDIR /app

# Copy all project files
COPY . .

# Build the application in release mode
RUN cargo build --release

# Stage 2: Runtime
FROM cgr.dev/chainguard/glibc-dynamic
WORKDIR /usr/local/bin

# Set nonroot user ownership for the copied binary
COPY --from=build --chown=nonroot:nonroot /app/target/release/${PACKAGE} ${PACKAGE}

# Use nonroot user to execute the binary
USER nonroot:nonroot

# Expose the default port (optional, for documentation)
EXPOSE 8080

# Pass environment variables (optional, modify as needed in production)
ENV BASE_URL=http://localhost:8080
ENV DATABASE_URL=mysql://shortener_user:s3cur3_p4ssw0rd@127.0.0.1:3306/url_shortener
ENV HOST=127.0.0.1
ENV PORT=8080

# Set the command to execute the binary
CMD ["/usr/local/bin/myapp"]
