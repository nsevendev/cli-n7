# n7 - CLI Nseven

CLI pour automatiser et simplifier les opérations Docker Compose de l'entreprise Nseven.

## Installation

### Installation rapide (recommandée)

```bash
curl -fsSL https://raw.githubusercontent.com/nsevendev/cli-n7/main/install.sh | bash
```

### Via Cargo

```bash
cargo install --git https://github.com/nsevendev/cli-n7
```

### Depuis les sources

```bash
git clone https://github.com/nseven/cli-n7
cd cli-n7
cargo install --path .
```

### Téléchargement manuel

Téléchargez le binaire pour votre plateforme depuis les [releases GitHub](https://github.com/nseven/cli-n7/releases/latest):

- **Linux x86_64**: `n7-linux-x86_64.tar.gz`
- **macOS Intel**: `n7-macos-x86_64.tar.gz`
- **macOS Apple Silicon**: `n7-macos-aarch64.tar.gz`

Puis extrayez et installez:

```bash
tar -xzf n7-*.tar.gz
sudo mv n7 /usr/local/bin/
```

## Utilisation

### Commandes disponibles

```bash
# Afficher l'aide
n7 --help
# ou
n7 -h
```

## Développement

### Prérequis

- Docker & Docker Compose
- Rust 1.91.1+ (si vous compilez en dehors du container)

### Setup avec Docker (recommandé)

```bash
# Lancer le container de développement
docker compose up -d

# Lancer le container de développement avec build
docker compose up --build -d

# Entrer dans le container
docker exec -it cli_n7_rust_dev bash

# Dans le container utiliser les outils directement
cargo test
cargo fmt
cargo clippy
```

### Conventional Commits

Pour que release-please fonctionne correctement, utilisez les **Conventional Commits**:

```bash
# Feature (bump version MINOR)
git commit -m "feat: add new docker compose down command"

# Bug fix (bump version PATCH)
git commit -m "fix: correct env file resolution"

# Breaking change (bump version MAJOR)
git commit -m "feat!: change command structure"
# ou
git commit -m "feat: change API BREAKING CHANGE: command structure has changed"

# Chore (pas de bump)
git commit -m "chore: update dependencies"

# Documentation (pas de bump)
git commit -m "docs: update README with installation instructions"

# etc ... voir liste si dessous
```

### Types de commits supportés

- `feat:` - Nouvelle fonctionnalité (MINOR)
- `fix:` - Correction de bug (PATCH)
- `docs:` - Documentation seulement
- `style:` - Formatage, points-virgules manquants, etc.
- `refactor:` - Refactoring du code
- `perf:` - Amélioration des performances
- `test:` - Ajout de tests
- `chore:` - Maintenance, mise à jour des dépendances
- `ci:` - Changements dans la CI

## License

MIT

## Contributing

Les contributions sont les bienvenues ! N'oubliez pas d'utiliser les conventional commits.
