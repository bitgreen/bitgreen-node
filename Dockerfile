# Docker image for bitgreen parachain image

# This is a base image to build substrate nodes
FROM docker.io/paritytech/ci-linux:production as builder

WORKDIR /bitgreen-node
COPY . .
RUN cargo build -p bitgreen-parachain --locked --release

# This is the 2nd stage: a very small image where we copy the binary."
FROM docker.io/library/ubuntu:20.04
LABEL description="Docker image for bitgreen parachain" \
  image.type="builder" \
  image.authors="team@bitgreen.org" \
  image.vendor="Bitgreen" \
  image.description="Docker image for bitgreen parachain"

# Copy the node binary.
COPY --from=builder /bitgreen-node/target/release/bitgreen-parachain /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /node-dev node-dev && \
  mkdir -p /chain-data /node-dev/.local/share && \
  chown -R node-dev:node-dev /chain-data && \
  ln -s /chain-data /node-dev/.local/share/bitgreen-node && \
  # unclutter and minimize the attack surface
  rm -rf /usr/bin /usr/sbin && \
  # check if executable works in this container
  /usr/local/bin/bitgreen-parachain --version

USER node-dev

EXPOSE 30333 9933 9944 9615
VOLUME ["/chain-data"]

ENTRYPOINT ["/usr/local/bin/bitgreen-parachain"]