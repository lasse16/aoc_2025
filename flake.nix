{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs?ref=nixos-25.05";
    # naersk does not provide releases
    naersk = {
      url = "github:nix-community/naersk/master";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    naersk,
    ...
  } @ inputs: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {inherit system;};
    naersk-lib = pkgs.callPackage naersk {};
    nix-specific-build-inputs = with pkgs; [glib pkg-config];
  in {
    packages.${system}.default = naersk-lib.buildPackage {
      src = ./.;
      nativeBuildInputs = nix-specific-build-inputs;
    };
    devShells.${system}.default = with pkgs;
      mkShell {
        buildInputs = [cargo rustc rustfmt rustPackages.clippy];
        nativeBuildInputs = nix-specific-build-inputs;
        env.RUST_SRC_PATH = rustPlatform.rustLibSrc;
      };
  };
}
