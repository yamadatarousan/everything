use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use std::process::{self, Command, Stdio};
use std::time::Duration;

pub struct DaemonManager {
    pid_file: PathBuf,
}

impl DaemonManager {
    pub fn new() -> Result<Self> {
        let pid_file = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("everything")
            .join("daemon.pid");

        // pidファイルのディレクトリを作成
        if let Some(parent) = pid_file.parent() {
            std::fs::create_dir_all(parent)?;
        }

        Ok(DaemonManager { pid_file })
    }

    pub fn start_daemon(&self) -> Result<()> {
        if self.is_running()? {
            println!("デーモンは既に実行中です");
            return Ok(());
        }

        println!("デーモンを起動中...");
        
        let current_exe = std::env::current_exe()?;
        let child = Command::new(current_exe)
            .arg("--watch")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        // PIDファイルに書き込み
        fs::write(&self.pid_file, child.id().to_string())?;
        
        println!("デーモン起動完了 (PID: {})", child.id());
        println!("ファイル監視がバックグラウンドで実行されています");
        
        Ok(())
    }

    pub fn stop_daemon(&self) -> Result<()> {
        let pid = match self.get_daemon_pid()? {
            Some(pid) => pid,
            None => {
                println!("デーモンは実行されていません");
                return Ok(());
            }
        };

        println!("デーモンを停止中... (PID: {})", pid);
        
        // プロセスを終了
        #[cfg(unix)]
        {
            unsafe {
                libc::kill(pid as i32, libc::SIGTERM);
            }
        }
        
        #[cfg(windows)]
        {
            Command::new("taskkill")
                .args(&["/PID", &pid.to_string(), "/F"])
                .output()?;
        }

        // PIDファイルを削除
        if self.pid_file.exists() {
            fs::remove_file(&self.pid_file)?;
        }

        println!("デーモン停止完了");
        Ok(())
    }

    pub fn get_status(&self) -> Result<()> {
        match self.get_daemon_pid()? {
            Some(pid) => {
                if self.is_process_running(pid)? {
                    println!("デーモン実行中 (PID: {})", pid);
                } else {
                    println!("PIDファイルが残っていますが、プロセスは実行されていません");
                    fs::remove_file(&self.pid_file)?;
                }
            }
            None => {
                println!("デーモンは停止中です");
            }
        }
        Ok(())
    }

    pub fn is_running(&self) -> Result<bool> {
        match self.get_daemon_pid()? {
            Some(pid) => self.is_process_running(pid),
            None => Ok(false),
        }
    }

    fn get_daemon_pid(&self) -> Result<Option<u32>> {
        if !self.pid_file.exists() {
            return Ok(None);
        }

        let pid_str = fs::read_to_string(&self.pid_file)?;
        let pid = pid_str.trim().parse::<u32>()?;
        Ok(Some(pid))
    }

    fn is_process_running(&self, pid: u32) -> Result<bool> {
        #[cfg(unix)]
        {
            unsafe {
                let result = libc::kill(pid as i32, 0);
                Ok(result == 0)
            }
        }
        
        #[cfg(windows)]
        {
            let output = Command::new("tasklist")
                .args(&["/FI", &format!("PID eq {}", pid)])
                .output()?;
            let output_str = String::from_utf8_lossy(&output.stdout);
            Ok(output_str.contains(&pid.to_string()))
        }
    }
}