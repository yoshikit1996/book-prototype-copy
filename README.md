# book-prototype

### 開発ツールのインストール

```

# Rustのインストール

$ curl https://sh.rustup.rs -sSf | sh

# cargoをPATHに追加(非zsh環境の場合は適宜書き換える必要あり)
$ echo "export PATH=\$PATH:\$HOME/.cargo/bin" >> ~/.zprofile

$ rustup install 1.39.0
$ rustup default 1.39.0

# gRPCクライアントのインストール

$ brew install grpcurl

# テストランナー

$ cargo install cargo-watch
```

### 開発に使用するコマンド等

```
# Webサーバ起動
$ cargo run

# gRPCリクエスト送信
$ grpcurl -proto src/grpc/models.proto -d '{"name":"Kiryu Coco"}' -plaintext localhost:50051 user.UserService.NewUser

# テストを常に走らせる
$ cargo watch -x test
```
