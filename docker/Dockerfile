FROM gitpod/workspace-dotnet

RUN sudo apt-get update && \
    sudo apt-get install gettext libncurses5 libxkbcommon0 libtinfo5 -y

USER gitpod

RUN brew install httpie && \
    brew install bat && \
    brew install exa 

# ------------------------------------
# Install Rust support
# ------------------------------------
#RUN rustup toolchain uninstall stable-x86_64-unknown-linux-gnu && \
RUN curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh -s -- -y && \
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh && \
    rustup target add wasm32-wasi
