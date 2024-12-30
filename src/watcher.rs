use std::{ collections::HashSet, path::PathBuf, thread };
use notify::{ Config, RecommendedWatcher, RecursiveMode, Watcher, Event };
use anyhow;

pub fn wait_for_change<F>(watch_path: Vec<PathBuf>, callback: F) -> anyhow::Result<()>
    where F: Fn(Event) + Send + 'static
{
    let (tx, rx) = std::sync::mpsc::channel();
    // 配置 watcher
    let mut watcher = RecommendedWatcher::new(move |event| {
        // 捕获事件并发送到通道
        if tx.send(event).is_err() {
            eprintln!("Failed to send event to channel");
        }
    }, Config::default()).map_err(|e| anyhow::anyhow!("Failed to create watcher: {}", e))?;

    // 在单独线程中轮询事件
    thread::spawn(move || {
        // 启动对多个路径的监控
        for path in watch_path {
            if let Err(err) = watcher.watch(&path, RecursiveMode::Recursive) {
                eprintln!("Failed to watch path {:?}: {}", path, err);
            }
        }

        loop {
            match rx.recv() {
                Ok(event) => {
                    match event {
                        Ok(mut ev) => {
                            // 对事件中的 paths 进行去重
                            let mut unique_paths: HashSet<PathBuf> = HashSet::new();
                            ev.paths.retain(|path| unique_paths.insert(path.clone()));

                            callback(ev); // 调用回调传递去重后的事件
                        } // 调用回调传递事件
                        Err(err) => eprintln!("Watch error: {:?}", err),
                    }
                }
                Err(e) => {
                    eprintln!("Channel error: {:?}", e);
                    break; // 停止循环
                }
            }
        }
    });

    Ok(())
}
