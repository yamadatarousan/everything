# Everything - 高速ファイル検索ツール

🔍 Windows「Everything」ライクな高速ファイル検索CLIツール（Rust製）

![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg)

## 概要

Windows用ファイル検索ツール「Everything」のような高速検索を、Rust言語で実装したクロスプラットフォーム対応CLIツールです。SQLiteによる事前インデックス作成とリアルタイムファイル監視により、ホームディレクトリ配下の数万ファイルから瞬時に検索結果を返します。

### 特徴

- ⚡ **高速検索**: 事前インデックスによる瞬時検索
- 🔄 **リアルタイム更新**: ファイル変更の自動検出・反映
- 🖥️ **クロスプラットフォーム**: Windows/macOS/Linux対応
- 🛡️ **安全**: ホームディレクトリ配下のみ（システムファイル除外）
- 📦 **軽量**: 単一バイナリ、外部依存なし
- 🇯🇵 **日本語対応**: 完全日本語インターフェース

## インストール

### 1. リポジトリのクローン
```bash
git clone https://github.com/yamadatarousan/everything.git
cd everything
```

### 2. 自動インストール
```bash
./install.sh
```

### 3. PATH設定（永続化）
```bash
# zshの場合
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc

# bashの場合  
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

## 使用方法

### 基本コマンド

```bash
# ヘルプ表示
ev --help

# 初回インデックス作成（必須）
ev --update

# バックグラウンド監視開始
ev --daemon

# ファイル検索
ev "検索語"
```

### 検索例

```bash
# 基本検索（部分一致）
ev "config"

# 拡張子検索
ev ".rs"

# 大文字小文字区別
ev --case-sensitive "Main"

# 正規表現検索
ev --regex "test.*\.js$"
```

### システム管理

```bash
# デーモン状態確認
ev --status

# 統計情報表示
ev --stats

# デーモン停止
ev --stop

# インデックス更新
ev --update
```

## 主な機能

### 🔍 高速検索
- SQLiteベースの高速インデックス
- 部分一致・正規表現検索対応
- 大文字小文字区別オプション

### 📁 検索対象
- **ホームディレクトリ配下**: `~/`
- Documents, Downloads, Desktop, プロジェクトフォルダ等
- システムファイルは対象外（安全性確保）

### 🔄 リアルタイム監視
- ファイル作成・削除・変更の自動検出
- バックグラウンドデーモン実行
- PIDファイルによるプロセス管理

### ⚙️ 設定
- インデックスDB: `~/.local/share/everything/index.db`
- PIDファイル: `~/.local/share/everything/daemon.pid`

## パフォーマンス

**テスト環境**: macOS (SSD)
- **ファイル数**: 約94,000ファイル
- **データサイズ**: 約1.87GB
- **インデックス時間**: 約2分
- **検索速度**: 瞬時（<100ms）

## 技術仕様

### 使用技術
- **言語**: Rust 1.70+
- **データベース**: SQLite (rusqlite)
- **CLI**: clap
- **ファイル監視**: notify
- **並行処理**: tokio

### アーキテクチャ
```
src/
├── main.rs       # エントリーポイント
├── cli.rs        # CLI引数解析
├── database.rs   # SQLite操作
├── indexer.rs    # ファイルインデックス作成
├── search.rs     # 検索エンジン
├── watcher.rs    # リアルタイム監視
└── daemon.rs     # デーモン管理
```

## 開発

### ビルド
```bash
# 開発ビルド
cargo build

# リリースビルド
cargo build --release

# テスト実行
cargo test

# フォーマット
cargo fmt

# リント
cargo clippy
```

### 依存関係
主要な依存クレート：
- `clap` - CLI引数解析
- `rusqlite` - SQLiteデータベース
- `notify` - ファイル監視
- `walkdir` - ディレクトリ走査
- `anyhow` - エラーハンドリング

## トラブルシューティング

### よくある問題

**Q: 検索結果が出ない**
```bash
# インデックスが作成されているか確認
ev --stats

# インデックスを再作成
ev --update
```

**Q: 新しいファイルが検索されない**
```bash
# デーモンが動いているか確認
ev --status

# デーモンを開始
ev --daemon
```

**Q: パーミッションエラー**
- ホームディレクトリ配下のみアクセスするため、通常は発生しません
- 設定ディレクトリ（`~/.local/`）の権限を確認してください

## ライセンス

MIT License - 詳細は[LICENSE](LICENSE)ファイルを参照

## 作者

現場で使える汎用開発ツール集の一環として開発

## 貢献

Issue報告やPull Requestを歓迎します。

---

**類似ツール**: Windows「Everything」, macOS「Alfred」, Linux「locate/updatedb」
