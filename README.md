# Rust-Lang

Learn Rust Basic

## Install

Install rustup first via [rustup-init.exe](https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe)

Then install tool

```bash
$ rustup default stable-x86_64-pc-windows-gnu
```

Install nodejs and dependencies

```
$ npm install
```

## Run command

Create new project with `cargo`

```bash
$ cargo new ch1_helloworld
```

Run code
Note: some project using stdio::stdin().read_line may not work correctly.
Then use command `cd project_folder/ && cargo run && rm -rf target/ && cd ..`. Project folder like `ch2_guessing_game/`...

```bash
$ npm start ch1_helloworld/
```

## Vscode Extendsion

. [Crates](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates)

. [Better TOML](https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml)

. [Search-crates-io](https://marketplace.visualstudio.com/items?itemName=belfz.search-crates-io)

. [Cargo](https://marketplace.visualstudio.com/items?itemName=panicbit.cargo)

. [Debug](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)

. [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) (recommend) hoặc [Rust RLS](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)

. [ErrorLens](https://marketplace.visualstudio.com/items?itemName=usernamehw.errorlens)

Format on save. add setting below to `settings.json` - Enter `Ctrl + ,` and click to `Open Setting(JSON)` button

```json
{
  "[rust]": {
    "editor.defaultFormatter": "matklad.rust-analyzer",
    "editor.formatOnSave": true
  },
  "rust-analyzer.updates.askBeforeDownload": true,
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.inlayHints.enable": true,
  "rust-analyzer.inlayHints.chainingHints": true,
  "rust-analyzer.inlayHints.parameterHints": true,
  "rust-analyzer.rustfmt.enableRangeFormatting": true
}
```

Debug profile

```json
{
  // Use IntelliSense to learn about possible attributes.
  // Hover to view descriptions of existing attributes.
  // For more information, visit: https://go.microsoft.com/fwlink/?linkid=830387
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug",
      "program": "${workspaceFolder}/${file}",
      "args": [],
      "cwd": "${workspaceFolder}"
    }
  ]
}
```

## Tài liệu

**Basic**

. [The Rust Book](https://doc.rust-lang.org/stable/book/)
. [The Cargo Book](https://doc.rust-lang.org/cargo/index.html)
. [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

**Domain Specific**

. [The CLI Book](https://rust-cli.github.io/book/index.html)

. [The WebAssembly Book](https://rustwasm.github.io/docs/book/#how-to-read-this-book)

. [The Embeded Book](https://doc.rust-lang.org/stable/embedded-book/)

. [Một vài thử nghiệm về xây dựng hệ điều hành](https://cs140e.sergio.bz/). Tham khảo thêm CS50.

. [Xây dựng backend service từ con số 0](https://www.lpalmieri.com/posts/2020-05-24-zero-to-production-0-foreword/)

**Advances**

. [Rust Standard Library Ref](https://doc.rust-lang.org/std/index.html)

. [The Rust Ref](https://doc.rust-lang.org/reference/index.html)

. [The unsafe Book](https://doc.rust-lang.org/nightly/unstable-book/index.html)

. [The Rustonomicon](https://doc.rust-lang.org/nomicon/)

. [The Little Book of Rust Marcos](https://veykril.github.io/tlborm/introduction.html)

. [The Rust FFI Omnibus](https://jakegoulding.com/rust-ffi-omnibus/)

. [Channel of Jen Gjengset](https://www.youtube.com/channel/UC_iD0xppBwwsrM9DegC5cQQ) - [Crust Playlist](https://www.youtube.com/watch?v=rAl-9HwD858&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa)

**Hand On Practice**

. [Rustlings](https://github.com/rust-lang/rustlings)

. [Exercism.io](https://exercism.org/tracks/rust/exercises)

. [Awesome Rust](https://github.com/rust-unofficial/awesome-rust)
