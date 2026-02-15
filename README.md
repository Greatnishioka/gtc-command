# gtc

日本語で入力したコミットメッセージを英語に翻訳し、ターミナル上で編集してそのまま `git commit` するCLIです。

## 概要

`gtc "日本語メッセージ"` を実行すると、次の順番で処理します。

1. `trans` コマンドで英語に翻訳
2. 翻訳結果を1行入力欄に初期値として表示（編集可能）
3. Enter確定で `git commit -m "<編集後メッセージ>"` を実行

## 前提

- Rust（ビルド用）
- Git
- [translate-shell (`trans`)](https://github.com/soimort/translate-shell)

## 使い方

```bash
gtc "これはサンプルです"
```

またはローカル実行:

```bash
cargo run -- "これはサンプルです"
```

## インストール（ローカルビルド）

```bash
cargo build --release
```

生成されたバイナリ:

```bash
./target/release/gtc
```

必要に応じて `PATH` の通った場所に配置して使ってください。

## 動作仕様

- Gitリポジトリ外で実行するとエラー終了します
- 翻訳結果が空の場合はコミットしません
- 編集入力で `Ctrl+C` / `Ctrl+D` した場合は中断します
- 入力欄は見やすさのため黄色で表示されます
