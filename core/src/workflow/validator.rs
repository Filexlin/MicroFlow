use std::collections::{HashMap, HashSet};

/// 检测工作流中的循环依赖
/// edges: 边的集合，格式为 (from_node_id, to_node_id)
pub fn detect_cycles(edges: &[(String, String)]) -> Result<(), String> {
    // 构建邻接表
    let mut adjacency: HashMap<&String, Vec<&String>> = HashMap::new();
    for (from, to) in edges {
        adjacency.entry(from).or_default().push(to);
    }

    // 记录访问过的节点和当前路径
    let mut visited: HashSet<&String> = HashSet::new();
    let mut path: HashSet<&String> = HashSet::new();

    // 对每个未访问的节点进行深度优先搜索
    for (from, _) in edges {
        if !visited.contains(from) && has_cycle(from, &adjacency, &mut visited, &mut path) {
            return Err("工作流中存在循环依赖".to_string());
        }
    }

    Ok(())
}

/// 深度优先搜索检测循环
fn has_cycle<'a>(
    node: &'a String,
    adjacency: &HashMap<&'a String, Vec<&'a String>>,
    visited: &mut HashSet<&'a String>,
    path: &mut HashSet<&'a String>,
) -> bool {
    // 如果节点已经在当前路径中，说明存在循环
    if path.contains(node) {
        return true;
    }

    // 如果节点已经访问过，直接返回
    if visited.contains(node) {
        return false;
    }

    // 标记节点为已访问，并加入当前路径
    visited.insert(node);
    path.insert(node);

    // 递归访问相邻节点
    if let Some(neighbors) = adjacency.get(node) {
        for neighbor in neighbors {
            if has_cycle(neighbor, adjacency, visited, path) {
                return true;
            }
        }
    }

    // 从当前路径中移除节点
    path.remove(node);
    false
}

/// 验证类型匹配
/// from: 源节点类型
/// to: 目标节点类型
pub fn validate_type_match(from: &str, to: &str) -> Result<(), String> {
    // 简单的类型匹配规则
    // 实际应用中可能需要更复杂的类型系统
    let valid_matches = [
        ("input", "llm"),
        ("llm", "output"),
        ("input", "output"),
        ("llm", "llm"),
    ];

    if valid_matches.contains(&(from, to)) {
        Ok(())
    } else {
        Err(format!("类型不匹配: {} 不能连接到 {}", from, to))
    }
}
