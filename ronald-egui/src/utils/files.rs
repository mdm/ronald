use std::path::PathBuf;

use sha3::Digest;

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
    extensions: &[&str],
    picked_file: Shared<Option<File>>,
) {
    use std::thread::spawn;

    let title = title.to_string();
    let filter_name = filter_name.to_string();
    let extensions = extensions
        .iter()
        .flat_map(|extension| [extension.to_uppercase(), extension.to_string()])
        .collect::<Vec<_>>();

    spawn(move || {
        if let Some(file) = rfd::FileDialog::new()
            .set_title(title)
            .add_filter(filter_name, &extensions)
            .pick_file()
            && let Ok(image) = std::fs::read(&file)
        {
            picked_file.with_mut(|f| {
                *f = Some(File {
                    path_buf: file,
                    image,
                });
            });
        }
    });
}

#[cfg(target_arch = "wasm32")]
pub fn pick_file(
    title: &str,
    filter_name: &str,
    extensions: &[&str],
    picked_file: Shared<Option<File>>,
) {
    let title = title.to_string();
    let filter_name = filter_name.to_string();
    let extensions = extensions
        .iter()
        .flat_map(|extension| [extension.to_uppercase(), extension.to_string()])
        .collect::<Vec<_>>();

    wasm_bindgen_futures::spawn_local(async move {
        if let Some(file) = rfd::AsyncFileDialog::new()
            .set_title(title)
            .add_filter(filter_name, &extensions)
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
pub fn pick_multiple_files(
    title: &str,
    filter_name: &str,
    extensions: &[&str],
    picked_files: Shared<Vec<File>>,
) {
    use std::thread::spawn;

    let title = title.to_string();
    let filter_name = filter_name.to_string();
    let extensions = extensions
        .iter()
        .flat_map(|extension| [extension.to_uppercase(), extension.to_string()])
        .collect::<Vec<_>>();

    spawn(move || {
        if let Some(files) = rfd::FileDialog::new()
            .set_title(title)
            .add_filter(filter_name, &extensions)
            .pick_files()
        {
            let files = files
                .into_iter()
                .filter_map(|file| {
                    std::fs::read(&file).ok().map(|image| File {
                        path_buf: file,
                        image,
                    })
                })
                .collect::<Vec<_>>();
            picked_files.with_mut(|f| {
                *f = files;
            });
        }
    });
}

#[cfg(target_arch = "wasm32")]
pub fn pick_multiple_files(
    title: &str,
    filter_name: &str,
    extensions: &[&str],
    picked_files: Shared<Vec<File>>,
) {
    let title = title.to_string();
    let filter_name = filter_name.to_string();
    let extensions = extensions
        .iter()
        .flat_map(|extension| [extension.to_uppercase(), extension.to_string()])
        .collect::<Vec<_>>();

    wasm_bindgen_futures::spawn_local(async move {
        if let Some(handles) = rfd::AsyncFileDialog::new()
            .set_title(title)
            .add_filter(filter_name, &extensions)
            .pick_files()
            .await
        {
            let mut files = Vec::with_capacity(handles.len());
            for handle in handles {
                files.push(File {
                    path_buf: handle.file_name().into(),
                    image: handle.read().await,
                });
            }
            picked_files.with_mut(|f| {
                *f = files;
            });
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn download_file(url: &str, downloaded_file: Shared<Option<File>>) {
    use std::thread::spawn;

    let url = url.to_string();

    spawn(move || {
        if let Ok(image) = reqwest::blocking::get(&url)
            .and_then(|response| response.bytes())
            .map(|bytes| bytes.to_vec())
        {
            let path_buf = url::Url::parse(&url)
                .ok()
                .and_then(|url| {
                    url.path_segments()
                        .and_then(|mut segments| segments.next_back().map(|s| s.to_string()))
                })
                .unwrap_or_else(|| {
                    let hash = sha3::Sha3_256::digest(&image).to_vec();
                    let encoded = hex::encode(&hash);
                    format!("{encoded}.bin")
                })
                .into();

            downloaded_file.with_mut(|f| {
                *f = Some(File { path_buf, image });
            });
        }
    });
}

#[cfg(target_arch = "wasm32")]
pub fn download_file(url: &str, downloaded_file: Shared<Option<File>>) {
    let url = url.to_string();

    wasm_bindgen_futures::spawn_local(async move {
        let Ok(response) = reqwest::get(&url).await else {
            return;
        };

        let Ok(bytes) = response.bytes().await else {
            return;
        };

        let image = bytes.to_vec();
        let path_buf = url::Url::parse(&url)
            .ok()
            .and_then(|url| {
                url.path_segments()
                    .and_then(|mut segments| segments.next_back().map(|s| s.to_string()))
            })
            .unwrap_or_else(|| {
                let hash = sha3::Sha3_256::digest(&image).to_vec();
                let encoded = hex::encode(&hash);
                format!("{encoded}.bin")
            })
            .into();

        downloaded_file.with_mut(|f| {
            *f = Some(File { path_buf, image });
        });
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
