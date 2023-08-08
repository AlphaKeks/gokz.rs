{
	inputs = {
		nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
		utils.url = "github:numtide/flake-utils";
	};

	outputs = { nixpkgs, utils, ... }: utils.lib.eachDefaultSystem (
		system: let
			pkgs = import nixpkgs {
				inherit system;
			};
		in {
			devShell = pkgs.mkShell {
				nativeBuildInputs = with pkgs; [ rustup openssl pkg-config ];
				buildInputs = with pkgs; [ just cargo-watch ];
				shellHook = ''
					rustup toolchain install stable
					rustup toolchain install nightly
					rustup default stable
					rustup +nightly component add rustfmt
				'';
			};
		}
	);
}

