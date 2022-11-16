FROM nixos/nix

RUN echo "experimental-features = flakes nix-command" > /etc/nix/nix.conf
RUN nix profile install nixpkgs#cachix \
    && cachix use 3waffel

COPY . /project/
WORKDIR project

# build all deps
RUN nix build . -o ./run && nix-collect-garbage

CMD ["./run/bin/telegram-bot"]