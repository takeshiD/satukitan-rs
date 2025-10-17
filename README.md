# Satukitan-rs

Rustで実装した Satukitan スクリプト言語インタプリタです。数値・真偽値・文字列・リスト、制御構造 `nobu` や束縛 `gakas` / `gakasdenu`、算術・論理・比較などの組み込み関数を備え、REPL と `.sample.st` スクリプトの両方を実行できます。

## インストール

### 前提
- Rust 1.78 以降（`rustup` で最新 stable を推奨）
- Git

### 手順
```bash
# リポジトリを取得
git clone <your-fork-or-clone-url>
cd satukitan-rs

# 実行ファイルをインストール（~/.cargo/bin に satukitan が入ります）
cargo install --path .
```

システムにインストールせずに試す場合は `cargo run -- <cmd>` を利用してください。

## 使い方

### REPL を起動する
```bash
cargo run -- repl
```
```
satukitan> gakasu x ra
satukitan> x
ra
satukitan> quit
```

### スクリプトを実行する
`run` サブコマンドに `.sample.st` ファイルを渡します。
```bash
cargo run -- run examples/hello.sample.st
```
または、`cargo install` 済みなら
```bash
satukitan run examples/hello.sample.st
```

## チュートリアル

### 1. リテラルと演算
```sat
ritas ra ru      # 2 + 1 => ro (3)
kenus ga me      # false or true => me
fanitas [ro ra ru]  # [3 2 1] -> [1 2 3]
```
REPL では式を入力すると評価結果が表示されます。

### 2. 変数束縛
```sat
gakas message "sana sapotav!"
message
```
`gakas` は変数名と値を受け取り、カレント環境に束縛します。

### 3. 関数定義と呼び出し
Satukitan では関数定義に `gakasdenu` を用います。第一引数が関数名、第二引数が引数リスト、第三引数が本体です。
```sat
gakasdenu add-two (x y) (
    ritas x y
)
add-two ra ru   # => ro
```
本体は複数の式を並べられ、最後の式の値が返されます。

### 4. 条件分岐
```sat
nobu (ditas n ru)    # if n < 1
    (ru)             # then
    (ritas n ru)     # else
```
`nobu` は `条件 then式 else式` の順で記述します。条件は真偽値を返す式で、真の場合は第二引数が、偽の場合は第三引数が評価されます。

### 5. 再帰関数の例: フィボナッチ
`examples/fibonacci.sample.st` には以下の例が含まれています。
```sat
gakasdenu fibo (n) (
    nobu (ditasgata n ru)
        (n)
        (ritas (fibo (matyes n ru)) (fibo (matyes n ra)))
)

sipus (fibo rya)   # => ryo (8)
```
実行:
```bash
cargo run -- run examples/fibonacci.sample.st
```

### 6. 標準出力
`sipus` は受け取った値を表示します。
```sat
sipus "hello"
sipus (ritas ra ru)
```

## テスト
開発時は次のコマンドでフォーマットと検証を行ってください。
```bash
cargo fmt
cargo clippy --all-targets
cargo test
```

## ライセンス
MIT License
