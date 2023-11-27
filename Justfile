build-colors:
    deno run build/main.ts > src/colors.rs
    cargo fmt
