{rustPlatform}:
rustPlatform.buildRustPackage {
  name = "ratatui-counter-app";
  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;
}
