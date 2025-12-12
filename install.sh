#!/bin/bash
set -e

# Couleurs pour l'affichage
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
REPO="nsevendev/cli-n7"  # Remplacez par votre repo GitHub (owner/repo)
BIN_NAME="n7"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

# Fonction pour afficher les messages
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

# Détection de l'OS et de l'architecture
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
            error "OS non supporté: $os"
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
            error "Architecture non supportée: $arch"
            ;;
    esac

    PLATFORM="${OS}-${ARCH}"
    info "Plateforme détectée: ${PLATFORM}"
}

# Récupérer la dernière version
get_latest_version() {
    info "Récupération de la dernière version..."

    VERSION=$(curl -s "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

    if [ -z "$VERSION" ]; then
        error "Impossible de récupérer la dernière version"
    fi

    info "Dernière version: ${VERSION}"
}

# Télécharger et installer le binaire
download_and_install() {
    local download_url="https://github.com/${REPO}/releases/download/${VERSION}/${BIN_NAME}-${PLATFORM}.tar.gz"
    local tmp_dir=$(mktemp -d)
    local tmp_file="${tmp_dir}/${BIN_NAME}.tar.gz"

    info "Téléchargement depuis: ${download_url}"

    if ! curl -fsSL -o "${tmp_file}" "${download_url}"; then
        error "Échec du téléchargement"
    fi

    info "Extraction..."
    tar -xzf "${tmp_file}" -C "${tmp_dir}"

    # Créer le répertoire d'installation si nécessaire
    mkdir -p "${INSTALL_DIR}"

    info "Installation dans ${INSTALL_DIR}..."
    mv "${tmp_dir}/${BIN_NAME}" "${INSTALL_DIR}/${BIN_NAME}"
    chmod +x "${INSTALL_DIR}/${BIN_NAME}"

    # Nettoyage
    rm -rf "${tmp_dir}"

    info "Installation terminée!"
}

# Vérifier si le répertoire d'installation est dans le PATH
check_path() {
    if [[ ":$PATH:" != *":${INSTALL_DIR}:"* ]]; then
        warn "Le répertoire ${INSTALL_DIR} n'est pas dans votre PATH"
        echo ""
        echo "Ajoutez cette ligne à votre ~/.bashrc ou ~/.zshrc :"
        echo ""
        echo "    export PATH=\"${INSTALL_DIR}:\$PATH\""
        echo ""
    else
        info "Le binaire est installé et disponible dans votre PATH"
    fi
}

# Afficher les instructions finales
show_completion() {
    echo ""
    echo -e "${GREEN}✓ Installation réussie!${NC}"
    echo ""
    echo "Pour commencer à utiliser ${BIN_NAME}, tapez:"
    echo ""
    echo "    ${BIN_NAME} --help"
    echo ""
}

# Main
main() {
    echo "═══════════════════════════════════════"
    echo "  Installation de ${BIN_NAME}"
    echo "═══════════════════════════════════════"
    echo ""

    detect_platform
    get_latest_version
    download_and_install
    check_path
    show_completion
}

main "$@"
