curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh


"$HOME/.cargo/env"            # For sh/bash/zsh/ash/dash/pdksh
source "$HOME/.cargo/env.fish"  # For fish
source "$HOME/.cargo/env.nu"    # For nushell

rustc --version
cargo --version
rustup --version
rustup update
which rustc
cargo --help

# 
cargo init .
cargo init --lib
# cargo new <project_name>
cargo new ffoo
# cargo new --lib <projectname>
cargo new --lib lib_project
