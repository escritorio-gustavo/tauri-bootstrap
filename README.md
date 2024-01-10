# Tauri + Solid + Typescript + Tailwind

This template should help get you started developing with Tauri, Solid, Typescript and TailwindCSS in Vite.

This also includes ESLint, PostCSS, Prettier and some of the Rust crates I use most often

## Usage

1. Clone the repo by running `git clone git@github.com:escritorio-gustavo/tauri-bootstrap.git`
1. Run `mv tauri-bootstrap <NEW_NAME>` to rename the directory
1. Open the project directory in your editor of choice (in VS Code's case, run `code <NEW_NAME>`)
1. Replace all instances of `{{app_name}}` by the name of your project. They will be in the files:
   - `package.json`
   - `src-tauri/Cargo.toml`
   - `src-tauri/Cargo.lock`
   - `src-tauri/tauri.conf.json`
1. Run `pnpm install` (or `pnpm update` to get the current versions of the packages)
1. Run `pnpm tauri dev`

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
