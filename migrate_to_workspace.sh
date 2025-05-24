#!/usr/bin/env bash
set -e

# This script migrates a single-crate project into a Cargo workspace layout.
# It is designed to be idempotent and can be run multiple times.

#---------------------------------------
# 1. Initialize workspace
#---------------------------------------
if ! grep -q "\[workspace\]" Cargo.toml 2>/dev/null; then
  cat > Cargo.toml <<'TOML'
[workspace]
members = ["schema", "schema_derive", "app"]
TOML
  echo "Created root Cargo.toml with workspace members."
else
  echo "Workspace root already exists, skipping creation."
fi

#---------------------------------------
# 2. Extract the trait crate
#---------------------------------------
if [ ! -d schema ]; then
  mkdir -p schema/src
  cat > schema/Cargo.toml <<'TOML'
[package]
name = "schema"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
TOML
  echo "Created schema crate."
fi

if [ -f src/lib.rs ] && [ ! -f schema/src/lib.rs ]; then
  mv src/lib.rs schema/src/lib.rs
  echo "Moved src/lib.rs to schema crate."
fi

#---------------------------------------
# 3. Update proc-macro crate
#---------------------------------------
if [ ! -d schema_derive ]; then
  mkdir -p schema_derive/src
  echo "Created schema_derive crate directory."
fi
cat > schema_derive/Cargo.toml <<'TOML'
[package]
name = "schema_derive"
version = "0.1.0"
edition = "2021"

[lib]
proc-macro = true

[dependencies]
syn = { version = "1.0", features = ["full"] }
quote = "1.0"
proc-macro2 = "1.0"
schema = { path = "../schema" }
TOML

echo "Configured schema_derive crate."

#---------------------------------------
# 4. Extract the application
#---------------------------------------
if [ ! -d app ]; then
  cargo init --bin app
  echo "Initialized app crate."
fi

if [ -f src/main.rs ] && [ ! -f app/src/main.rs ]; then
  mv src/main.rs app/src/main.rs
  echo "Moved src/main.rs to app crate."
fi

cat > app/Cargo.toml <<'TOML'
[package]
name = "app"
version = "0.1.0"
edition = "2021"

[dependencies]
schema = { path = "../schema" }
schema_derive = { path = "../schema_derive" }
serde = { version = "1.0", features = ["derive"] }
TOML

echo "Configured app crate."

#---------------------------------------
# 5. Standardize editions
#---------------------------------------
for crate in schema schema_derive app; do
  if [ -f "$crate/Cargo.toml" ]; then
    sed -i '/^edition\s*=\s*/d' "$crate/Cargo.toml"
    sed -i '/\[package\]/a edition = "2021"' "$crate/Cargo.toml"
  fi
done

echo "Ensured edition 2021 in all crates."

#---------------------------------------
# 6. Verify workspace build
#---------------------------------------
if command -v cargo >/dev/null 2>&1; then
  cargo build
else
  echo "cargo not found; skipping build."
fi

#---------------------------------------
# 7. Future libraries reminder
#---------------------------------------
cat <<'MSG'
# To add a new library crate:
cargo new --lib my_new_crate --vcs none
MSG
