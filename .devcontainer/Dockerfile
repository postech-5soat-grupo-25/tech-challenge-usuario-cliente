# Use the official Rust image from Docker Hub
FROM rustlang/rust:nightly

# Install essential tools and dependencies
RUN apt-get update && \
    apt-get install -y \
    build-essential \
    gdb \
    curl \
    sudo \
    fonts-firacode \
    && \
    rm -rf /var/lib/apt/lists/*

# Install Rust Analyzer
RUN curl -L https://github.com/rust-analyzer/rust-analyzer/releases/latest/download/rust-analyzer-linux -o /usr/local/bin/rust-analyzer \
    && chmod +x /usr/local/bin/rust-analyzer

# Install Starship prompt
RUN curl -fsSL https://starship.rs/install.sh | sh -s -- -y

# Set up Starship configuration
RUN mkdir ~/.config
RUN echo 'eval "$(starship init bash)"' >> ~/.bashrc
RUN starship preset nerd-font-symbols -o ~/.config/starship.toml

# Configure font for terminal
ENV DEBIAN_FRONTEND=noninteractive
RUN echo 'export LANG=C.UTF-8' >> ~/.bashrc \
    && echo 'export LC_ALL=C.UTF-8' >> ~/.bashrc \
    && echo 'export TERM=xterm-256color' >> ~/.bashrc \
    && echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc \
    && echo 'export EDITOR=vim' >> ~/.bashrc \
    && echo 'export TZ="America/Sao_Paulo"' >> ~/.bashrc

# Set up the workspace
WORKDIR /workspace
