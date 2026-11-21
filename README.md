# Zaphkiel VCC

Listing VCC (VRChat Creator Companion) généré depuis les releases GitHub, servi par un binaire Rust qui embarque le front Astro.

## Structure

| Chemin      | Rôle                                                                              |
| ----------- | --------------------------------------------------------------------------------- |
| `apps/api`  | Serveur `axum` : interroge GitHub en tâche de fond, sert `/index.json` en mémoire |
| `apps/web`  | Page Astro statique, compilée puis embarquée dans le binaire                      |
| `examples/` | Workflow GitHub Actions à copier dans chaque dépôt de package                     |

La racine porte le workspace Cargo (`Cargo.toml`, `Cargo.lock`) et le workspace Bun (`package.json`, `bun.lock`).

## Endpoints

| URL                         | Contenu                                                           |
| --------------------------- | ----------------------------------------------------------------- |
| `/index.json`               | Listing complet                                                   |
| `/index.json?p=com.a,com.b` | Listing limité à ces packages, avec un `id` propre à la sélection |
| `/`                         | Page de sélection des packages                                    |

## Configuration

| Variable          | Défaut                     | Rôle                                                                  |
| ----------------- | -------------------------- | --------------------------------------------------------------------- |
| `REPOS`           | obligatoire                | Dépôts GitHub séparés par des virgules (lien complet ou `owner/repo`) |
| `GITHUB_TOKEN`    | vide                       | Passe la limite de l'API GitHub de 60 à 5000 requêtes par heure       |
| `LISTING_NAME`    | `Zaphkiel VCC`             | Nom du listing dans le VCC                                            |
| `LISTING_AUTHOR`  | `Zaphkiel`                 | Auteur du listing                                                     |
| `LISTING_ID`      | `dev.zaphkiel.vcc`         | Identifiant du listing                                                |
| `PUBLIC_URL`      | `https://vcc.zaphkiel.dev` | URL publique, écrite dans le champ `url` du listing                   |
| `REFRESH_SECONDS` | `300`                      | Intervalle d'interrogation de GitHub                                  |
| `PORT`            | `8080`                     | Port d'écoute                                                         |

## Format attendu des releases

Une release est publiée dans le listing si elle contient en assets :

- `package.json` : le manifeste du package, avec `name` et une `version` semver valide ;
- un `.zip` du package, de préférence nommé `<name>-<version>.zip`.

Les brouillons sont ignorés. `examples/release.yml` produit ce format pour un dépôt dont le `package.json` est à la racine.

## Lancer

```sh
cp .env.example .env
docker compose up -d --build
```

## Développement

```sh
bun install
bun run build
REPOS=owner/repo cargo run
bun run dev
```

`bun run build` doit précéder `cargo run` : l'API embarque `apps/web/dist` à la compilation. `bun run dev` relaie `/index.json` vers l'API sur le port 8080.

| Commande                     | Rôle                                                   |
| ---------------------------- | ------------------------------------------------------ |
| `bun run check`              | Vérification des types Astro                           |
| `bun run lint`               | oxlint, toutes catégories en erreur, règles type-aware |
| `bun run format`             | Formatage oxfmt                                        |
| `cargo fmt --all`            | Formatage Rust                                         |
| `cargo clippy --all-targets` | clippy `pedantic`, `nursery` et restrictions en `deny` |
