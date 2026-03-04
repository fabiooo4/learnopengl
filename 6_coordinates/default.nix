{
  rustPlatform,
  glfw3,
  pkg-config,
  xorg,
  wayland,
}:
rustPlatform.buildRustPackage {
  name = "coordinates";
  src = ./.;
  buildInputs = [
    glfw3
    xorg.libX11
    wayland
  ];
  nativeBuildInputs = [pkg-config];
  cargoLock = {
    lockFile = ./Cargo.lock;
  };
}
