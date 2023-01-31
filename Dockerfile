# linux image with golang, python and lua
FROM ubuntu:latest



# install golang
RUN apt-get update
# install curl
RUN apt-get install -y curl
# install python
RUN apt install -y llvm-dev libclang-dev clang


# install cmake
RUN apt-get install -y cmake

# Get Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

COPY . ./app/
WORKDIR /app

# CMD ["/bin/bash"]