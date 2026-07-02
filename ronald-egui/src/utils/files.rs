use std::path::PathBuf;

use crate::utils::sync::{Shared, SharedExt};

#[derive(Debug)]
pub struct File {
    pub path_buf: PathBuf,
    pub image: Vec<u8>,
}

#[cfg(not(target_arch = "wasm32"))]
pub fn pick_file(
    title: &str,
    filter_name: &str,
    extension: &str,
    picked_file: Shared<Option<File>>,
) {
    use std::thread::spawn;

    let title = title.to_string();
    let filter_name = filter_name.to_string();
    let extension = extension.to_string();
    spawn(move || {
        if let Some(file) = rfd::FileDialog::new()
            .set_title(&title)
            .add_filter(&filter_name, &[&extension])
            .pick_file()
        {
            if let Ok(image) = std::fs::read(&file) {
                picked_file.with_mut(|f| {
                    *f = Some(File {
                        path_buf: file,
                        image,
                    });
                });
            }
        }
    });
}

#[cfg(target_arch = "wasm32")]
pub fn pick_file(
    title: &str,
    filter_name: &str,
    extension: &str,
    picked_file: Shared<Option<File>>,
) {
    let title = title.to_string();
    let filter_name = filter_name.to_string();
    let extension = extension.to_string();
    wasm_bindgen_futures::spawn_local(async move {
        if let Some(file) = rfd::AsyncFileDialog::new()
            .set_title(&title)
            .add_filter(&filter_name, &[&extension])
            .pick_file()
            .await
        {
            let file_data = File {
                path_buf: file.file_name().into(),
                image: file.read().await,
            };
            picked_file.with_mut(|f| {
                *f = Some(file_data);
            });
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn pick_folder(title: &str, picked_folder: Shared<Option<PathBuf>>) {
    use std::thread::spawn;

    let title = title.to_string();
    spawn(move || {
        if let Some(folder) = rfd::FileDialog::new().set_title(&title).pick_folder() {
            picked_folder.with_mut(|f| {
                *f = Some(folder);
            });
        }
    });
}
