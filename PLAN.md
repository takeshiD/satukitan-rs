# Satukitan 実装計画

## 言語仕様概要
- 構文スタイル: Lisp系プレフィックス記法。式は関数名の後に引数を並べ、必要に応じて丸括弧またはリスト記号`[]`を用いる。
- リテラル:
  - 数値: `rv`(0)〜`rye`(9), `#ta`(10)を基底に、今後は連結記法や拡張子を検討。現状は固定語を整数として解釈する。
  - 真偽値: `ga`(false), `me`(true)。
  - 文字列: 二重引用符で囲むUTF-8文字列。
- コンテナ:
  - リスト: `[`と`]`で囲み、要素を空白で区切る。`fanitas`(ソート), `rakas`(長さ)などの組み込みで操作。
- 演算/関数:
  - 算術: `ritas`(+), `matyes`(-), `nitas`(*)。
  - 論理: `teses`(and), `kenus`(or)。
  - 比較: `ditas`(<), `fityes`(>), `gatas`(==), `ditasgata`(<=), `fityesgata`(>=)。
  - 入出力: `sipus`で標準出力に値を表示。
- 制御構造:
  - `nobu` 条件分岐。第1引数が真なら第2式、偽なら第3式を評価。
- 束縛/関数定義:
  - `gakas` 変数束縛(値を環境へ登録)。
  - `gakasdenu` 関数束縛(再帰/非再帰を問わずラムダを環境へ登録)。関数定義は一律`gakasdenu`を使用する。
  - 関数適用はシンボルを先頭に置き、後続を引数として評価。
- 評価規則(想定): 正格評価。式は左から右へ評価し、関数適用時に引数を事前評価。ただし`nobu`や束縛系特殊形式は遅延評価で分岐引数の遅延を確保。
- エラー方針: 未束縛シンボル、型不一致、引数過不足、パース失敗時に人間可読なメッセージを返す。

## 予約語・予約シンボル
- リテラル: `rv`, `ru`, `ra`, `ro`, `re`, `ri`, `rya`, `ryu`, `ryo`, `rye`, `#ta`, `ga`, `me`。
- 特殊形式/組み込み関数: `nobu`, `gakas`, `gakasdenu`, `sipus`, `ritas`, `matyes`, `nitas`, `teses`, `kenus`, `ditas`, `fityes`, `gatas`, `ditasgata`, `fityesgata`, `fanitas`, `rakas`。
- 予約記号: `(`, `)`, `[`, `]`, `"`。これらは語として再定義不可。
- 今後の拡張余地として`コメント`記法や数値拡張語を追加予定。追加時はここに追記。

## アーキテクチャ設計
### 実行フロー
1. 入力(標準入力REPLまたは`.st`ファイル)を読み込む。
2. 字句解析: `lexer`でトークン列へ(空白・改行・括弧・文字列リテラル処理)。
3. 構文解析: `parser`(`nom`利用)でASTへ変換。
4. 評価: `evaluator`がASTを環境とともに走査し`Value`を生成。
5. ビルトイン/標準出力: `runtime`内の組み込みがRust側処理を提供。
6. 結果をREPLなら表示、スクリプトなら終了コードと標準出力を返す。

### モジュール構成(予定)
- `ast`: ノード定義(`Expr`, `Literal`, `List`, `Lambda`, `SpecialForm`等)とシリアライズ補助関数。
- `lexer`: `nom`の`recognize`/`take_while`系コンビネータでトークナイズ。文字列・記号のエスケープ処理を担当。
- `parser`: トップレベル`parse_expr`/`parse_program`を提供。括弧構造、リスト、シンボル、特殊形式を認識し`ast`を生成。
- `value`: 実行時値(`Number`, `Bool`, `String`, `List`, `Function`, `Builtin`, `Nil`)と表示処理。
- `env`: 環境チェーン(ハッシュマップ+親ポインタ)管理。可変/不変を両立させるため`Rc<RefCell<...>>`想定。
- `builtins`: 算術・論理・比較・リスト・I/OのRust実装。`Value::Builtin`として登録。
- `evaluator`: ASTを評価し、特殊形式(`nobu`, `gakas`, `gakasdenu`)を専用ロジックで処理。特に`gakasdenu`は関数クロージャを生成し環境へ登録する。
- `repl`: 行編集(必要なら`rustyline`等)と対話ループ。環境継続とエラーハンドリングを備える。
- `cli`: コマンドライン引数解析(例:`clap`)でREPLとファイル実行を切り替え、入出力処理を統括。
- `lib.rs`: コアロジックをライブラリとして公開し、`main.rs`は薄いCLIラッパーにする方針。

### データ構造・エラーハンドリング
- `NumberLiteral`は列挙語→整数のマップを持つ。拡張時は`HashMap<&'static str, i64>`などで柔軟性確保。
- 文字列は`String`、リストは`Vec<Value>`。
- エラー型は`SatukitanError`(Variant: `Parse`, `Eval`, `Type`, `Runtime`)を設計し、`Result<Value, SatukitanError>`で伝播。
- 位置情報: パーサから`Span`を伝達し、エラーに行・列情報を含めることを検討。

## 実装TODO
### フェーズ1: 基盤整備
- [ ] 依存追加: `nom`, `thiserror`(エラー), `clap`(CLI), `rustyline`(REPLオプション)を`Cargo.toml`に記載。
- [ ] プロジェクト再構成: `src/lib.rs`作成、モジュール骨格(`ast`, `lexer`, `parser`, `value`, `env`, `builtins`, `evaluator`, `repl`, `cli`)の雛形を配置。
- [ ] 言語仕様ドキュメント整備: `DESIGN.md`に評価規則/エラーポリシーと、関数定義が`gakasdenu`である点を追記。

### フェーズ2: パーサ実装
- [ ] 字句解析: `lexer`モジュールでトークン列生成、ユニットテストを作成。
- [ ] 構文解析: `parser`で式/リスト/特殊形式を`nom`で解析。ネスト対応とエラー復旧方針を実装。
- [ ] パース検証: `tests/parser.rs`を作成し、主要構文(`ritas`, `nobu`, `gakas`, `gakasdenu`, リスト)を網羅。

### フェーズ3: 評価器・環境
- [ ] `Value`/`Env`定義: 実行時値と環境チェーンを実装し、ディスプレイ用フォーマットを整備。
- [ ] 特殊形式実装: `nobu`, `gakas`(値束縛), `gakasdenu`(関数束縛)の評価ロジックを追加。`gakasdenu`は非再帰関数でも環境へ自己参照として登録できるようにする。
- [ ] 組み込み登録: 算術/論理/比較/リスト/`sipus`を`builtins`で実装し、起動時に環境へロード。
- [ ] エラー処理: `SatukitanError`と`Result`伝播、テストケースで確認。

### フェーズ4: エントリーポイント
- [ ] REPL: `repl`モジュールで読み->解析->評価->表示のループ。履歴/終了コマンド(`quit`)検討。
- [ ] ファイル実行: `.st`拡張子を受け取り、ファイル内容を評価して終了コードを返す。
- [ ] CLI統合: `clap`で`satukitan run <file>`/`satukitan repl`等のサブコマンドを設計。

### フェーズ5: 品質保証
- [ ] テスト拡充: 組み込み関数・再帰・条件分岐・非再帰関数(`gakasdenu`)の評価テストを追加。
- [ ] ドキュメント更新: `README.md`に使用例、REPL操作、ファイル実行例(`sample.st`)を記載。
- [ ] 形式検証: `cargo fmt`, `cargo clippy`, `cargo test`をCI想定のローカルコマンドとして整備。
- [ ] サンプルスクリプト: `examples/`または`sample/`に`hello_world.st`などを配置し、動作確認手順を明示。

### フェーズ6: 将来拡張メモ
- [ ] コメント構文(例: `;`以降無視)の導入検討。
- [ ] 数値リテラル拡張(多桁表現、負数、浮動小数)の仕様策定。
- [ ] モジュールシステムや外部関数FFIの要件調査。
