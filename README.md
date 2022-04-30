# mendan

よしかわ　と面談するためのウェブサービス

## 詳細

[よしかわたいき](https://github.com/yoshikawa) と面談するときに、面談を楽しくするサービスを立ち上げる。

### 機能

- [] hoge

## 必要条件

| Tool           |  Version |
| :------------- | -------: |
| Rust           |   1.60.0 |
| Node.js        |   18.0.0 |
| PostgreSQL     |     14.2 |
| Docker         | 20.10.14 |
| docker-compose |    2.4.1 |

## 使い方

### mendan のダウンロード

```sh
git clone git@github.com:yoshikawa/mendan.git
```

### 開発環境の立ち上げ

```sh
# 開発環境の起動(初回のみ)
make

# APIサーバの起動
make run
# フロントエンドの起動
make front
```

## 開発環境の削除

```sh
make docker/down ## Dockerコンテナを全てシャットダウン
docker system prune --all
```

## コントリビュート

バグレポートとプルリクエストは, [https://github.com/yoshikawa/mendan](https://github.com/yoshikawa/mendan) で歓迎されています.

このプロジェクトは, コラボレーションのための安全で居心地の良い場所となることを目的としており, 寄稿者は[寄稿者規約](http://contributor-covenant.org)の行動規範を順守することが期待されています.

## 行動規範

このプロジェクトのコードベース, 課題追跡システム, チャットルーム, メーリングリストでやり取りするすべての人は, [行動規範](./.github/CODE_OF_CONDUCT.md)に従うことが期待されています.

## ライセンス

[MIT License](./LICENSE)
