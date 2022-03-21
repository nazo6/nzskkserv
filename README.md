# nzskkserv

とあるSKKサーバ実装です。

## 使い方
```
cargo install nzskkserv-cli
nzskkserv-cli
```

一度起動すると設定ファイルができるのでそれを編集して辞書を追加します。
設定ファイルはLinuxでは`XDG_CONFIG_HOME/nzskkserv/config.toml`、Windowsでは`%APPDATA/Roaming/nzskkserv/config/config.toml`にあります。

設定例:
```toml
enable_google_cgi = true
server_encoding = "Utf8" # SKKクライアントと通信するときに使う文字コード
port = 2000 # 通信するポート。デフォルトでは1178番です。

[[dicts]]
url = "http://openlab.jp/skk/skk/dic/SKK-JISYO.L"
encoding = "Eucjp" # 辞書ファイルの文字コード

[[dicts]]
url = "https://raw.githubusercontent.com/uasi/skk-emoji-jisyo/master/SKK-JISYO.emoji.utf8"
encoding = "Utf8"
```

起動時に自動で指定されたurlから辞書をダウンロードします。また、`enable_google_cgi`が`true`になっている場合、
ローカル辞書の検索結果が0件だったときにそちらにフォールバックします。

省メモリ化とかそういうのはしていないので読み込んだ辞書の分だけメモリを使います。
