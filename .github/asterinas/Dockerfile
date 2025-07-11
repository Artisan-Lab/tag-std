FROM ubuntu:24.04

SHELL ["/bin/bash", "-c"]

RUN apt-get update && apt-get install -y curl xz-utils liblzma-dev make exfatprogs

# Install Nix package manager
# FIXME: Installing Nix as root is not supported in single-user mode.
RUN sh <(curl -L https://nixos.org/nix/install) --daemon --yes \
    && . /etc/profile.d/nix.sh \
    && nix-channel --add https://nixos.org/channels/nixos-25.05 nixpkgs \
    && nix-channel --update \
    && nix-env -iA nixpkgs.nixfmt \
    && rm /nix/var/nix/gcroots/auto/* \
    && nix-collect-garbage -d

COPY ./test /asterinas/test

WORKDIR /asterinas/test

RUN make -n

CMD . /etc/profile.d/nix.sh \
  && make build --trace && ls -alh && ls -alh build \
  && cp -rL build /tmp && ls -alh /tmp/build \
  && cp -rf /tmp/build/* /build
