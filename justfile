container := "docker"
device := "iPhone 18 Pro Max"
export PODMAN_COMPOSE_WARNING_LOGS := "false"

[continue]
bootstrap:
    @echo "Install blaze..."
    curl -fsSL https://ix.dev/blaze | sh
    @echo "Install target wasm32-unknown-unknown aarch64-apple-ios aarch64-apple-ios-sim..."
    rustup target add wasm32-unknown-unknown aarch64-apple-ios aarch64-apple-ios-sim
    @echo "pnpm install..."
    pnpm install
    pnpm exec tailwindcss -i crates/dioxus-components/tailwind.css -o crates/dioxus-components/assets/tailwind.css

[continue]
start container=container:
    cd deployments/docker && {{container}} compose --env-file ../../config/.env up -d

[continue]
stop container=container:
    cd deployments/docker && {{container}} compose --env-file ../../config/.env down --remove-orphans
    rm -rf deployments/docker/postgres
    rm -rf deployments/docker/redis
    rm -rf deployments/docker/iggy
    rm -rf deployments/docker/rustfs
    rm -rf deployments/docker/meilisearch
    rm -rf deployments/docker/openobserve

[continue]
cli:
    watchexec -r mbx run -j 4 -p cli help

[continue]
tui:
    watchexec -r mbx run -j 4 -p tui

[continue]
server: start
    #!/usr/bin/env bash
    trap 'true' INT
    watchexec -r mbx run -j 4 -p server
    status=$?
    just stop
    case $status in 0|130|143) exit 0;; *) exit $status;; esac

[continue]
web:
    dx serve -p web

[continue]
macos:
    dx serve -p desktop

[continue]
ios device=device:
    dx serve -p mobile --ios --device {{device}} --target aarch64-apple-ios-sim
