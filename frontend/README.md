# book

## 開発ツールのインストール

```shell
# node & npmのインストール
$ curl -L git.io/nodebrew | perl - setup$ nodenv install 13.8.0
# bashの人はよしなに対応してください
$ echo "export PATH=\$HOME/.nodebrew/current/bin:\$PATH" >> ~/.zprofile
$ source ~/.zprofile
$ nodebrew install v13.8.0
$ nodebrew use v13.8.0
```

## Project setup
```
npm install
```

### Compiles and hot-reloads for development
```
npm run serve
```

### Compiles and minifies for production
```
npm run build
```

### Run your unit tests
```
npm run test:unit
```

### Run your end-to-end tests
```
npm run test:e2e
```

### Lints and fixes files
```
npm run lint
```
