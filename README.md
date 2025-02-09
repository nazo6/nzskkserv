# nzskkserv

RustとUIフレームワークのDioxusで構築されたSKKサーバー実装。

## Feature

- [x] SKKサーバープロトコル対応
  - [x] 基本プロトコル(`0`-`3`)
  - [ ] `4`: 補完
  - [ ] 様々なエッジケース対応
  - [ ] lisp関数対応?
- [x] GUI
  - [x] 変換ログ
  - [x] 設定
  - [ ] 変換統計
- [x] 自動起動
- [x] Google CGIサーバー経由での変換
- [x] 辞書の読み込み
  - [x] SKK形式
  - [x] mozc形式
- [x] URLからの辞書のダウンロード
  - [ ] 辞書のアップデート
- [ ] OS対応
  - [x] Windows

## Building

```sh
cargo binstall dioxus-cli
pnpm i
pnpm build
# or
pnpm bundle
```

## Config

設定はGUIで行える他、`%APPDATA%/Roaming/nzskkserv/config/config.toml`に保存されるファイルを編集することでも行えます。以下は設定例です。

```toml
enable_google_cgi = true
server_encoding = "Utf8"
port = 1178

[[dicts]]
url = "http://openlab.jp/skk/skk/dic/SKK-JISYO.L"
encoding = "Eucjp"
format = "Skk"

[[dicts]]
url = "https://raw.githubusercontent.com/uasi/skk-emoji-jisyo/master/SKK-JISYO.emoji.utf8"
encoding = "Utf8"
format = "Skk"

[[dicts]]
url = "https://raw.githubusercontent.com/ncaq/dic-nico-intersection-pixiv/master/public/dic-nico-intersection-pixiv-google.txt"
encoding = "Utf8"
format = "Mozc"
```
