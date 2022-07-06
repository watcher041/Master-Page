
# Master-Page

- ソースをバンドルするためのパッケージ（webpackのようなもの）
```bash
cargo install trunk
```

- アプリにトレイトを追加するためのパッケージ
```bash
cargo install cargo-edit
```

- wasmをコンパイルしたときの出力形式（target）を追加する。
```bash
rustup target add wasm32-unknown-unknown
```

- アプリの作成
```bash
mkdir Master-Page
cd Master-Page
cargo init
```

- アプリを起動して、「Hello World!」が表示されることを確認
```bash
cargo run
```

- JSXで記載するフロントフレームワーク yew を導入
```bash
cargo add yew
```

- 最低限、動作するようにファイルを編集する

【src/main.rs】
```rust
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hello World" }</h1>
    }
}

fn main() {
    yew::start_app::<App>();
}
```

【index.html】
```html
<!DOCTYPE html>

<html>
  <head></head>
  <body></body>
</html>
```

- ファイルをバンドルして、サイトが表示されることを確認
```
trunk serve --open
```

- URLによって表示されるサイトを変更

```
cargo add yew-router
```