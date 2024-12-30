#![deny(clippy::all)]

mod watcher;
mod route_context;
mod file_utils;
mod cache_manager;

use farmfe_core::{ config::Config, context::CompilationContext, plugin::Plugin };
use farmfe_macro_plugin::farm_plugin;
use serde::Deserialize;
use std::{ path::PathBuf, sync::{ Arc, Mutex } };
use notify::EventKind;

#[farm_plugin]
#[derive(Clone)]
pub struct FarmRustPluginAutoRoutes {
    current_dir: PathBuf,
    route_context: route_context::RouteContext,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IOpts {
    // mode: String,
    // index_path: String,
}

impl FarmRustPluginAutoRoutes {
    fn new(_config: &Config, _options: String) -> Self {
        let current_dir = std::env::current_dir().unwrap();
        let mut route_context = route_context::RouteContext::new();
        route_context.init_routes();

        Self { current_dir, route_context }
    }
}

impl Plugin for FarmRustPluginAutoRoutes {
    fn name(&self) -> &str {
        "FarmRustPluginAutoRoutes"
    }

    fn build_start(
        &self,
        _context: &Arc<CompilationContext>
    ) -> farmfe_core::error::Result<Option<()>> {
        let watch_paths = vec![self.current_dir.join("src/pages")];
        let route_generator = Arc::new(Mutex::new(self.route_context.clone()));
        let _ = watcher::wait_for_change(watch_paths, move |event| {
            let mut route_generator_inner = route_generator.lock().unwrap();
            match event.kind {
                EventKind::Create(_) => {
                    // 新增文件
                    route_generator_inner.generate_routes_on_create(event.paths.clone());
                }
                EventKind::Modify(_) => {
                    // 修改文件
                    for path in event.paths {
                        if path.exists() {
                            route_generator_inner.generate_routes_on_modify(path);
                        } else {
                            route_generator_inner.generate_routes_on_remove(path);
                        }
                    }
                }
                _ => {}
            }
        });

        Ok(None)
    }
}
