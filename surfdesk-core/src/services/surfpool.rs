use std::process::Stdio;

use anyhow::Result;
use async_trait::async_trait;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::{Child, Command},
};

#[async_trait]
pub trait TraitSurfpool {
    async fn start(&mut self) -> Result<()>;
    async fn stop(&mut self) -> Result<()>;
    async fn is_running(&self) -> bool;
    async fn write(&mut self, input: &str) -> Result<()>;
    async fn read(&mut self) -> Result<String>;
}

#[derive(Debug)]
pub struct Surfpool {
    pub process: Option<Child>,
    pub status: SurfpoolStatus,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum SurfpoolStatus {
    #[default]
    Stop,
    Running,
}

impl Default for Surfpool {
    fn default() -> Self {
        Self {
            process: None,
            status: SurfpoolStatus::Stop,
        }
    }
}

impl Surfpool {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl TraitSurfpool for Surfpool {
    async fn start(&mut self) -> Result<()> {
        let mut cmd = Command::new("surfpool");
        cmd.arg("start")
            .arg("--no-tui")
            .arg("--debug")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let child = cmd.spawn()?;
        self.status = SurfpoolStatus::Running;
        self.process = Some(child);
        Ok(())
    }

    async fn stop(&mut self) -> Result<()> {
        if let Some(mut child) = self.process.take() {
            let _ = child.kill().await; // ป้องกัน panic กรณีโปรเซสปิดไปแล้ว
        }
        self.status = SurfpoolStatus::Stop;
        Ok(())
    }

    async fn is_running(&self) -> bool {
        self.status == SurfpoolStatus::Running
    }

    async fn write(&mut self, _input: &str) -> Result<()> {
        // จะ implement ต่อได้ถ้ามี stdin
        Ok(())
    }

    async fn read(&mut self) -> Result<String> {
        if let Some(child) = &mut self.process {
            if let Some(stdout) = child.stdout.as_mut() {
                let mut reader = BufReader::new(stdout);
                let mut line = String::new();
                let bytes = reader.read_line(&mut line).await?;
                if bytes > 0 {
                    return Ok(line.trim().to_string());
                }
            }
        }
        Ok(String::from("No output"))
    }
}
