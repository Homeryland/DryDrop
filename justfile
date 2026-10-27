container := "docker"
export PODMAN_COMPOSE_WARNING_LOGS := "false"

bootstrap:
  curl -fsSL https://ix.dev/blaze | sh
  rustup target add wasm32-unknown-unknown aarch64-apple-ios aarch64-apple-ios-sim
  pnpm install
  pnpm exec tailwindcss -i crates/dioxus-components/tailwind.css -o crates/dioxus-components/assets/tailwind.css

start container=container:
  cd deployments/docker && {{container}} compose --env-file ../../config/.env.dev up -d

stop container=container:
  cd deployments/docker && {{container}} compose --env-file ../../config/.env.dev down

cli:
  cargo blaze run -p cli

tui:
  cargo blaze run -p tui

server:
  cargo blaze run -p server

web:
  dx run -p web

macos:
  dx run -p desktop

ios:
  dx run -p mobile --ios
