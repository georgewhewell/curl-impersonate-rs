let
  nixpkgs = import <nixpkgs> { };
in
with nixpkgs;

pkgs.mkShell {
  buildInputs = with pkgs; [
    curl-impersonate-bin
  ];
  RUST_LOG = "info";
}
