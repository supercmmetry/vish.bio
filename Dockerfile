FROM rustlang/rust:nightly AS builder

# No --toolchain flag: the base image pins a dated toolchain (e.g.
# nightly-2026-08-12-x86_64-unknown-linux-gnu), so naming "nightly" here would install a
# *second* toolchain and add the target to that one, leaving the active toolchain without
# musl — cargo then fails with E0463 "can't find crate for `core`".
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app

RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash; \
    . /root/.bashrc; \
    nvm install 18; \
    nvm use 18; \
    . /root/.bashrc; \
    npm i -g yarn;

COPY Cargo.toml Cargo.lock package.json yarn.lock ./

RUN . /root/.bashrc; \
    yarn install

COPY . .

RUN . /root/.bashrc; \
    yarn build

RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM scratch

COPY --from=builder /usr/local/cargo/bin/vish-bio .

USER 1000

CMD ["./vish-bio"]