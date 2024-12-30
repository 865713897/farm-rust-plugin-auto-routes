use std::path::{ PathBuf, Component, Path };
use std::cmp::min;
use std::collections::BTreeMap;
use std::fs;
use serde_json::{ json, Value };
use regex::Regex;

use crate::cache_manager::Cache;

lazy_static::lazy_static! {
    static ref ROUTE_PATH_REGEX: Regex = Regex::new(r"(\/?index)?\.(tsx?|jsx?)$").unwrap();
}

pub fn parse_routes(
    file_list: &[String],
    base_path: &Path,
    write_path: &Path,
    meta_cache: &mut Cache<String, Value>
) -> (Vec<String>, Vec<String>) {
    let mut routes = Vec::new();
    let mut route_components = Vec::new();
    for file in file_list {
        let file_path = PathBuf::from(file);
        let route_path = file_path_to_route_path(&file_path, base_path);
        let route_id = if route_path.is_empty() {
            "index".to_string()
        } else {
            route_path.replace("/", "-")
        };

        let relative_path = get_relative_path(write_path, &file_path);
        let meta_file = file.replace(".tsx", ".meta.json").replace(".jsx", ".meta.json");
        let metadata = meta_cache.get_or_insert_with(&meta_file, || {
            if PathBuf::from(&meta_file).exists() {
                let content = fs::read_to_string(&meta_file).unwrap();
                let json_value: Value = serde_json::from_str(&content).unwrap_or(Value::Null);

                // 将 metadata 转换为有序结构
                if let Value::Object(obj) = json_value {
                    let ordered_obj: BTreeMap<_, _> = obj.into_iter().collect();
                    Value::Object(ordered_obj.into_iter().collect())
                } else {
                    json_value
                }
            } else {
                Value::Null
            }
        });
        let updated_metadata = match metadata.clone() {
            Value::Object(mut obj) => {
                obj.insert("id".to_string(), json!(route_id));
                obj.insert("path".to_string(), json!(format!("/{}", route_path)));
                Value::Object(obj) // 返回更新后的对象
            }
            _ =>
                json!({
                "id": route_id,
                "path": format!("/{}", route_path),
            }),
        };
        routes.push(
            format!(
                "'{}': {}",
                route_id,
                serde_json::to_string(&updated_metadata).unwrap().replace("\"", "'")
            )
        );
        route_components.push(
            format!(
                "      '{}': withLazyLoad(React.lazy(() => import('{}')))",
                route_id,
                relative_path
            )
        );
    }

    (routes, route_components)
}

fn file_path_to_route_path(file_path: &Path, prefix: &Path) -> String {
    let relative_path = file_path.strip_prefix(prefix).unwrap().to_str().unwrap();
    ROUTE_PATH_REGEX.replace_all(relative_path, "").to_string()
}

pub fn scan_directory(dir: PathBuf) -> Vec<String> {
    let mut path_list = Vec::new();

    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => {
            return path_list;
        }
    };

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                path_list.extend(scan_directory(path));
            } else {
                path_list.push(path.to_string_lossy().to_string());
            }
        }
    }

    path_list
}

// 路径转换成vec
pub fn path_to_components(path: &Path) -> Vec<String> {
    path.components()
        .filter_map(|component| {
            match component {
                Component::Normal(part) => Some(part.to_string_lossy().into_owned()),
                _ => None,
            }
        })
        .collect()
}

// 获取相对路径
pub fn get_relative_path(from: &Path, to: &Path) -> String {
    let from_components = path_to_components(from);
    let to_components = path_to_components(to);

    // 找到共同的前缀
    let mut i = 0;
    while
        i < min(from_components.len(), to_components.len()) &&
        from_components[i] == to_components[i]
    {
        i += 1;
    }

    // 计算相对路径
    let up_steps = from_components.len() - i;
    let down_steps = &to_components[i..];

    // 生成相对路径
    let mut relative_path = "../".repeat(up_steps);
    if up_steps == 0 {
        relative_path.push_str("./");
    }
    relative_path.push_str(&down_steps.join("/"));

    relative_path
}
