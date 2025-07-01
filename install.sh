#!/bin/bash

# Everything - 高速ファイル検索ツール インストールスクリプト

set -e

echo "=== Everything インストール開始 ==="

# ビルド
echo "リリースバイナリをビルド中..."
cargo build --release

# バイナリパス
BINARY_PATH="./target/release/everything"
INSTALL_DIR="$HOME/.local/bin"

# インストールディレクトリ作成
mkdir -p "$INSTALL_DIR"

# バイナリをコピー
echo "バイナリを $INSTALL_DIR にコピー中..."
cp "$BINARY_PATH" "$INSTALL_DIR/everything"
cp "$BINARY_PATH" "$INSTALL_DIR/ev"  # 短縮コマンド

# 実行権限付与
chmod +x "$INSTALL_DIR/everything"
chmod +x "$INSTALL_DIR/ev"

echo "インストール完了！"
echo ""
echo "=== 使用方法 ==="
echo "1. PATHに $INSTALL_DIR を追加してください："
echo "   export PATH=\"\$HOME/.local/bin:\$PATH\""
echo ""
echo "2. シェル設定ファイルに追加（永続化）："
if [[ "$SHELL" == *"zsh"* ]]; then
    echo "   echo 'export PATH=\"\$HOME/.local/bin:\$PATH\"' >> ~/.zshrc"
    echo "   source ~/.zshrc"
elif [[ "$SHELL" == *"bash"* ]]; then
    echo "   echo 'export PATH=\"\$HOME/.local/bin:\$PATH\"' >> ~/.bashrc"
    echo "   source ~/.bashrc"
else
    echo "   echo 'export PATH=\"\$HOME/.local/bin:\$PATH\"' >> ~/.profile"
    echo "   source ~/.profile"
fi
echo ""
echo "3. 使用例："
echo "   everything --help    # ヘルプ表示"
echo "   ev --help            # 短縮コマンド"
echo "   ev --daemon          # デーモン開始"
echo "   ev \"検索語\"          # ファイル検索"
echo ""
echo "=== セットアップが必要な場合 ==="
echo "初回使用時はインデックス作成が必要です："
echo "   ev --update          # インデックス作成"
echo "   ev --daemon          # バックグラウンド監視開始"
echo ""