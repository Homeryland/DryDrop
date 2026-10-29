container := "docker"
device := "iPhone 18 Pro Max"
export MBX_BLAZE := "1"
export PODMAN_COMPOSE_WARNING_LOGS := "false"

bootstrap:
    @echo "Install blaze..."
    curl -fsSL https://ix.dev/blaze | sh
    @echo "Install target wasm32-unknown-unknown aarch64-apple-ios aarch64-apple-ios-sim..."
    rustup target add wasm32-unknown-unknown aarch64-apple-ios aarch64-apple-ios-sim
    @echo "pnpm install..."
    pnpm install
    pnpm exec tailwindcss -i crates/dioxus-components/tailwind.css -o crates/dioxus-components/assets/tailwind.css

start container=container:
    rm -rf postgres
    rm -rf redis
    rm -rf iggy
    rm -rf rustfs
    rm -rf meilisearch
    rm -rf openobserve
    cd deployments/docker && {{container}} compose --env-file ../../config/.env.dev up -d

stop container=container:
    cd deployments/docker && {{container}} compose --env-file ../../config/.env.dev down

cli:
    watchexec -r mbx run -j 4 -p cli

tui:
    watchexec -r mbx run -j 4 -p tui

server:
    watchexec -r mbx run -j 4 -p server

web:
    dx serve -p web

macos:
    dx serve -p desktop

ios device=device:
    dx serve -p mobile --ios --device {{device}} --target aarch64-apple-ios-sim
