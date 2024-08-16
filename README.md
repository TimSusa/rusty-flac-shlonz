# Tauri SvelteKit 4 TeamIT
<img width="1316" alt="Screenshot 2024-04-23 at 22 42 16" src="https://github.com/teamitfi/tauri-svelte-example/assets/8963529/706bd3ed-6038-478f-90e7-f56d79ba3fb3">

- **Tauri**
- **SvelteKit**
- **GitHub action** for cross-platform builds
- **TypeScript**
- **Preprocessing** with Sass installed by default
- **ESLint**
- **Prettier**

## Dev instructions

### Get started

1. Install Node.js
2. Install Rust
3. Follow the [Tauri setup guide](https://tauri.app/v1/guides/getting-started/setup)
4. Run `npm install`


### Commands
- `npm run dev`: Start app in dev mode
- `npm run build`: Build the app to a native executable
- `npm run lint`: Lint
- `npm run format`: Format

### Release new version
1. Update `CHANGELOG.md`
2. Bump the version number in `src-tauri/Cargo.toml`
3. Run `cargo check` to update `Cargo.lock`
4. Create a git tag in the format `v#.#.#`
5. Add release notes to the generated GitHub release and publish it


## Prepare Database
Sqlite is used here along with an ORM called [Diesel](https://diesel.rs/).


```
# Linux/MacOS
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/download/v2.2.1/diesel_cli-installer.sh | sh

# Windows
powershell -c "irm https://github.com/diesel-rs/diesel/releases/download/v2.2.1/diesel_cli-installer.ps1 | iex"
```

open new terminal in your root folder and dropping:


```
diesel setup
diesel migration generate create_posts
```

After this, a folder called migrations should be created and containing a file up.sql.

## Migration Schema

Put this into the up.sql file

```
CREATE TABLE posts (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 0
)
``` 

and this into down.sql

```
DROP TABLE posts
``` 

then create the table by:

``` 
diesel migration run
```



## Build and Run

```
cargo build
cargo run <Path to your Flac Music Collection>
```

