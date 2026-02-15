use std::path::{Path, PathBuf};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PythonScript {
    pub name: String,
    pub description: String,
    pub relative_path: String,
    pub category: String,
}

pub struct PythonLibrary {
    root_path: PathBuf,
    scripts: HashMap<String, PythonScript>,
}

impl PythonLibrary {
    pub fn new(root_path: impl AsRef<Path>) -> Self {
        Self { root_path: root_path.as_ref().to_path_buf(), scripts: HashMap::new() }
    }
    
    pub fn scan(&mut self) -> Result<(), std::io::Error> {
        self.scripts.clear();
        if !self.root_path.exists() { std::fs::create_dir_all(&self.root_path)?; return Ok(()); }
        
        for entry in walkdir::WalkDir::new(&self.root_path)
            .into_iter().filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map(|ext| ext == "py").unwrap_or(false))
        {
            if let Some(script) = self.parse_script(entry.path()) {
                self.scripts.insert(script.relative_path.clone(), script);
            }
        }
        Ok(())
    }
    
    fn parse_script(&self, path: &Path) -> Option<PythonScript> {
        let content = std::fs::read_to_string(path).ok()?;
        let relative_path = path.strip_prefix(&self.root_path).ok()?.to_string_lossy().replace('\\', "/");
        let category = path.parent().and_then(|p| p.file_name()).map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        let default_name = path.file_stem()?.to_string_lossy().to_string();
        
        Some(PythonScript {
            name: Self::parse_meta(&content, "name").unwrap_or(default_name),
            description: Self::parse_meta(&content, "description").unwrap_or_default(),
            relative_path,
            category,
        })
    }
    
    fn parse_meta(content: &str, key: &str) -> Option<String> {
        content.lines().take(30).find_map(|line| {
            let line = line.trim();
            line.strip_prefix(&format!("# @{} ", key)).map(|s| s.to_string())
        })
    }
    
    pub fn get_by_category(&self) -> HashMap<String, Vec<&PythonScript>> {
        let mut cat: HashMap<String, Vec<&PythonScript>> = HashMap::new();
        for script in self.scripts.values() { cat.entry(script.category.clone()).or_default().push(script); }
        cat
    }
}
