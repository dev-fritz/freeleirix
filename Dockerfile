# Use the official Rust image as the base image
FROM rust:1.80

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY . .

# Copy the .env file
COPY .env .env

# Load environment variables from .env file
RUN export $(cat .env | xargs)

# Set the command to run the application with cargo-watch
CMD ["cargo", "run"]