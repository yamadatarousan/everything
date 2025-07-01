use anyhow::Result;
use everything::*;
use std::path::PathBuf;
use std::time::Duration;

fn main() -> Result<()> {
    let matches = cli::build_cli().get_matches();

    let db_path = dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("everything")
        .join("index.db");

    // データベースディレクトリを作成
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let db = Database::new(&db_path)?;

    if matches.get_flag("update") {
        println!("インデックスを更新中...");
        let indexer = Indexer::new(db);
        // テスト用に現在のディレクトリをインデックス
        let current_dir = std::env::current_dir()?;
        indexer.build_index(&current_dir.to_string_lossy())?;
        println!("インデックス更新完了");
        return Ok(());
    }

    if matches.get_flag("stats") {
        let (count, total_size) = db.get_stats()?;
        println!("ファイル数: {}", count);
        println!("総サイズ: {} bytes", total_size);
        return Ok(());
    }

    let daemon_manager = DaemonManager::new()?;

    if matches.get_flag("daemon") {
        daemon_manager.start_daemon()?;
        return Ok(());
    }

    if matches.get_flag("stop") {
        daemon_manager.stop_daemon()?;
        return Ok(());
    }

    if matches.get_flag("status") {
        daemon_manager.get_status()?;
        return Ok(());
    }

    if matches.get_flag("watch") {
        println!("ファイル監視モード開始...");
        let current_dir = std::env::current_dir()?;
        
        let (mut watcher, rx) = FileWatcher::new(db)?;
        watcher.watch(&current_dir)?;
        
        println!("監視中: {} (Ctrl+Cで終了)", current_dir.display());
        println!("ファイルの作成・削除・変更を自動的にインデックスに反映します");
        
        loop {
            match rx.recv_timeout(Duration::from_millis(100)) {
                Ok(Ok(event)) => {
                    if let Err(e) = watcher.handle_event(event) {
                        eprintln!("イベント処理エラー: {}", e);
                    }
                }
                Ok(Err(e)) => {
                    eprintln!("ファイル監視エラー: {}", e);
                }
                Err(_) => {
                    // タイムアウト（正常）
                }
            }
        }
    }

    if let Some(query) = matches.get_one::<String>("query") {
        let case_sensitive = matches.get_flag("case-sensitive");
        let results = db.search_files(query, case_sensitive)?;
        
        for result in results.iter().take(100) {
            println!("{}", result);
        }
        
        if results.len() > 100 {
            println!("... および他 {} 件", results.len() - 100);
        }
    } else {
        println!("使用方法: everything [検索語] または everything --help");
    }

    Ok(())
}
