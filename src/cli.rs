use clap::{Arg, Command};

pub fn build_cli() -> Command {
    Command::new("everything")
        .about("🔍 高速ファイル検索CLIツール - Windows「Everything」ライク")
        .version("0.1.0")
        .long_about("
Windows「Everything」のような高速ファイル検索を提供するRust製CLIツールです。
SQLiteによる事前インデックス作成とリアルタイムファイル監視により、
ホームディレクトリ配下のファイルから瞬時に検索結果を返します。

検索対象: ホームディレクトリ配下 (~/)
- Documents, Downloads, Desktop, プロジェクトフォルダ等
- システムファイルは対象外（安全性とパフォーマンスのため）

使用例:
  everything \"検索語\"              # 基本検索
  everything \"*.rs\"               # 拡張子検索  
  everything --case-sensitive \"Main\" # 大文字小文字区別
  everything --regex \"test.*\"      # 正規表現検索
  everything --daemon               # バックグラウンド監視開始
  everything --update               # インデックス更新
  
初回使用時: everything --update でインデックスを作成してください。
常時監視: everything --daemon でバックグラウンド監視を開始できます。
")
        .arg(
            Arg::new("query")
                .help("検索するファイル名やパス（部分一致）")
                .long_help("検索クエリを指定します。ファイル名やパスの一部を入力してください。\n例: \"config\", \"main.rs\", \"src/\"")
                .index(1)
                .required(false),
        )
        .arg(
            Arg::new("regex")
                .long("regex")
                .short('r')
                .help("正規表現検索を有効にする")
                .long_help("正規表現によるパターンマッチング検索を行います。\n例: --regex \"test.*\\.js$\"")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("case-sensitive")
                .long("case-sensitive")
                .short('c')
                .help("大文字小文字を区別する")
                .long_help("大文字と小文字を区別して検索します。デフォルトは区別しません。")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("update")
                .long("update")
                .short('u')
                .help("ファイルインデックスを更新する")
                .long_help("ファイルシステムをスキャンしてインデックスを再構築します。\n初回使用時や手動更新時に実行してください。")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("stats")
                .long("stats")
                .short('s')
                .help("インデックス統計情報を表示する")
                .long_help("インデックスされているファイル数と総サイズを表示します。")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("watch")
                .long("watch")
                .short('w')
                .help("フォアグラウンドでファイル監視する")
                .long_help("ファイルの変更をリアルタイムで監視し、インデックスを自動更新します。\nCtrl+Cで終了できます。")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("daemon")
                .long("daemon")
                .short('d')
                .help("バックグラウンドでファイル監視を開始する")
                .long_help("デーモンプロセスとしてバックグラウンドでファイル監視を実行します。\n常時リアルタイム検索を利用したい場合に便利です。")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("stop")
                .long("stop")
                .help("実行中のデーモンプロセスを停止する")
                .long_help("バックグラウンドで実行中のデーモンプロセスを停止します。")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("status")
                .long("status")
                .help("デーモンプロセスの実行状態を確認する")
                .long_help("デーモンプロセスが実行中かどうかとPIDを表示します。")
                .action(clap::ArgAction::SetTrue),
        )
}