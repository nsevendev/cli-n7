#!/bin/bash
set -e

# Colors for display
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
REPO="nsevendev/cli-n7"  # Replace with your GitHub repo (owner/repo)
BIN_NAME="n7"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

# Functions to display messages
info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
    exit 1
}

# Detect OS and architecture
detect_platform() {
    local os=$(uname -s | tr '[:upper:]' '[:lower:]')
    local arch=$(uname -m)

    case "$os" in
        linux)
            OS="linux"
            ;;
        darwin)
            OS="macos"
            ;;
        *)
            error "Unsupported OS: $os"
            ;;
    esac

    case "$arch" in
        x86_64|amd64)
            ARCH="x86_64"
            ;;
        aarch64|arm64)
            ARCH="aarch64"
            ;;
        *)
            error "Unsupported architecture: $arch"
            ;;
    esac

    PLATFORM="${OS}-${ARCH}"
    info "Detected platform: ${PLATFORM}"
}

# Get latest version
get_latest_version() {
    info "Fetching latest version..."

    VERSION=$(curl -s "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

    if [ -z "$VERSION" ]; then
        error "Unable to fetch latest version"
    fi

    info "Latest version: ${VERSION}"
}

# Download and install binary
download_and_install() {
    local download_url="https://github.com/${REPO}/releases/download/${VERSION}/${BIN_NAME}-${PLATFORM}.tar.gz"
    local tmp_dir=$(mktemp -d)
    local tmp_file="${tmp_dir}/${BIN_NAME}.tar.gz"

    info "Downloading from: ${download_url}"

    if ! curl -fsSL -o "${tmp_file}" "${download_url}"; then
        error "Download failed"
    fi

    info "Extracting..."
    tar -xzf "${tmp_file}" -C "${tmp_dir}"

    # Create installation directory if needed
    mkdir -p "${INSTALL_DIR}"

    info "Installing to ${INSTALL_DIR}..."
    mv "${tmp_dir}/${BIN_NAME}" "${INSTALL_DIR}/${BIN_NAME}"
    chmod +x "${INSTALL_DIR}/${BIN_NAME}"

    # Cleanup
    rm -rf "${tmp_dir}"

    info "Installation completed!"
}

# Check if installation directory is in PATH
check_path() {
    if [[ ":$PATH:" != *":${INSTALL_DIR}:"* ]]; then
        warn "The directory ${INSTALL_DIR} is not in your PATH"
        echo ""
        echo "Add this line to your ~/.bashrc or ~/.zshrc:"
        echo ""
        echo "    export PATH=\"${INSTALL_DIR}:\$PATH\""
        echo ""
    else
        info "Binary is installed and available in your PATH"
    fi
}

# Display final instructions
show_completion() {
    echo ""
    echo -e "${GREEN}✓ Installation successful!${NC}"
    echo ""
    echo "To start using ${BIN_NAME}, type:"
    echo ""
    echo "    ${BIN_NAME} --help"
    echo ""
}

# Main
main() {
    echo ""
    echo "    ███╗   ██╗███████╗"
    echo "    ████╗  ██║╚════██║"
    echo "    ██╔██╗ ██║    ██╔╝"
    echo "    ██║╚██╗██║   ██╔╝"
    echo "    ██║ ╚████║   ██║"
    echo "    ╚═╝  ╚═══╝   ╚═╝"
    echo ""
    echo "    Install ${BIN_NAME}"
    echo ""

    detect_platform
    get_latest_version
    download_and_install
    check_path
    show_completion
}

main "$@"
