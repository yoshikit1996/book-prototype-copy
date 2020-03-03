# book-prototype

### 開発ツールのインストール

```

# Rustのインストール

$ curl https://sh.rustup.rs -sSf | sh

# cargoをPATHに追加(非zsh環境の場合は適宜書き換える必要あり)
$ echo "export PATH=\$PATH:\$HOME/.cargo/bin" >> ~/.zprofile

$ rustup install 1.41.1
$ rustup default 1.41.1

# gRPCクライアントのインストール

$ brew install grpcurl

# テストランナー

$ cargo install cargo-watch

# PostgreSQL

$ brew install postgresql 
```

### 開発に使用するコマンド等

```
# Webサーバ起動
$ cargo run

# gRPCリクエスト送信
$ sh ./scripts/grpc_request.sh

# テストを常に走らせる
$ cargo watch -x test

# PostgreSQL接続
$ psql -h localhost -U root
```
