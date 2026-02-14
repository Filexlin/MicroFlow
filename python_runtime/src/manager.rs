use std::io::{BufReader, BufWriter};
use std::process::{Child, ChildStdin, ChildStdout, Command};
use std::time::{Duration, Instant};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PythonManagerError {
    #[error("进程启动失败: {0}")]
    ProcessStartError(String),

    #[error("进程池已满")]
    PoolFull,

    #[error("无可用进程")]
    NoAvailableProcess,

    #[error("进程已终止: {0}")]
    ProcessTerminated(usize),
}

pub struct PythonProcess {
    pub id: usize,
    pub child: Child,
    pub stdin: BufWriter<ChildStdin>,
    pub stdout: BufReader<ChildStdout>,
    pub last_used: Instant,
    pub is_available: bool,
}

impl PythonProcess {
    pub fn new(id: usize, python_path: &str) -> Result<Self, PythonManagerError> {
        let mut child = Command::new(python_path)
            .arg("-m")
            .arg("microflow_runtime")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::null())
            .spawn()
            .map_err(|e| PythonManagerError::ProcessStartError(e.to_string()))?;

        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| PythonManagerError::ProcessStartError("无法获取 stdin".to_string()))?;

        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| PythonManagerError::ProcessStartError("无法获取 stdout".to_string()))?;

        Ok(Self {
            id,
            child,
            stdin: BufWriter::new(stdin),
            stdout: BufReader::new(stdout),
            last_used: Instant::now(),
            is_available: true,
        })
    }

    pub fn is_alive(&mut self) -> bool {
        match self.child.try_wait() {
            Ok(Some(_)) => false,
            Ok(None) => true,
            Err(_) => false,
        }
    }

    pub fn kill(&mut self) -> Result<(), std::io::Error> {
        self.child.kill()
    }
}

pub struct PythonManager {
    pool: Vec<PythonProcess>,
    max_pool_size: usize,
    #[allow(dead_code)]
    timeout: Duration,
    python_path: String,
    next_process_id: usize,
}

impl PythonManager {
    pub fn new(max_pool_size: usize, timeout: Duration, python_path: &str) -> Self {
        Self {
            pool: Vec::with_capacity(max_pool_size),
            max_pool_size,
            timeout,
            python_path: python_path.to_string(),
            next_process_id: 0,
        }
    }

    pub fn start(&mut self) -> Result<(), PythonManagerError> {
        // 预启动一半的进程池容量
        let pre_start_count = self.max_pool_size / 2;
        for _ in 0..pre_start_count {
            self.create_process()?;
        }
        Ok(())
    }

    pub fn stop(&mut self) {
        for process in &mut self.pool {
            let _ = process.kill();
        }
        self.pool.clear();
    }

    pub fn get_process(&mut self) -> Result<&mut PythonProcess, PythonManagerError> {
        // 清理僵尸进程
        self.cleanup_zombie_processes();

        // 查找可用进程的索引
        let mut available_idx = None;
        for (idx, process) in self.pool.iter_mut().enumerate() {
            if process.is_available && process.is_alive() {
                available_idx = Some(idx);
                break;
            }
        }

        // 如果找到可用进程
        if let Some(idx) = available_idx {
            let process = &mut self.pool[idx];
            process.is_available = false;
            process.last_used = Instant::now();
            return Ok(process);
        }

        // 如果没有可用进程，尝试创建新进程
        if self.pool.len() < self.max_pool_size {
            self.create_process()?;
            let idx = self.pool.len() - 1;
            let process = &mut self.pool[idx];
            process.is_available = false;
            return Ok(process);
        }

        Err(PythonManagerError::NoAvailableProcess)
    }

    pub fn release_process(&mut self, process_id: usize) {
        if let Some(process) = self.pool.iter_mut().find(|p| p.id == process_id) {
            process.is_available = true;
            process.last_used = Instant::now();
        }
    }

    pub fn restart_process(&mut self, process_id: usize) -> Result<(), PythonManagerError> {
        // 查找并终止旧进程
        let old_process_idx = self.pool.iter().position(|p| p.id == process_id);
        if let Some(idx) = old_process_idx {
            let _ = self.pool[idx].kill();
            self.pool.remove(idx);
        }

        // 创建新进程
        self.create_process()?;
        Ok(())
    }

    pub fn cleanup_zombie_processes(&mut self) {
        // 创建一个新的列表来保存活跃的进程
        let mut alive_processes = Vec::new();

        // 检查每个进程是否活跃
        for mut process in self.pool.drain(..) {
            if process.is_alive() {
                alive_processes.push(process);
            }
        }

        // 替换为活跃进程列表
        self.pool = alive_processes;
    }

    pub fn health_check(&mut self) {
        // 清理僵尸进程
        self.cleanup_zombie_processes();

        // 重启长时间未使用的进程（超过 10 分钟）
        let ten_minutes = Duration::from_secs(600);
        let now = Instant::now();

        // 收集需要重启的进程 ID
        let mut to_restart = Vec::new();
        for process in &self.pool {
            if now.duration_since(process.last_used) > ten_minutes {
                to_restart.push(process.id);
            }
        }

        // 重启进程
        for &process_id in &to_restart {
            let _ = self.restart_process(process_id);
        }
    }

    fn create_process(&mut self) -> Result<(), PythonManagerError> {
        if self.pool.len() >= self.max_pool_size {
            return Err(PythonManagerError::PoolFull);
        }

        let process = PythonProcess::new(self.next_process_id, &self.python_path)?;
        self.next_process_id += 1;
        self.pool.push(process);
        Ok(())
    }
}
