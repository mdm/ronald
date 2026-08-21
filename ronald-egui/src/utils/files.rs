use std::path::PathBuf;

use crate::utils::sync::{Shared, SharedExt};

#[derive(Debug)]
pub struct File {
    pub path_buf: PathBuf,
    pub image: Vec<u8>,
}

#[cfg(not(target_arch = "wasm32"))]
pub fn pick_file(title: &str, filters: &[(&str, &str)], picked_file: Shared<Option<File>>) {
    use std::thread::spawn;

    let title = title.to_string();
    let filters = filters
        .iter()
        .map(|(name, extension)| (name.to_string(), extension.to_string()))
        .collect::<Vec<_>>();

    spawn(move || {
        let mut dialog = rfd::FileDialog::new().set_title(title);

        for (filter_name, extension) in filters.into_iter() {
            dialog = dialog.add_filter(filter_name, &[extension]);
        }

        if let Some(file) = dialog.pick_file()
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
pub fn pick_file(title: &str, filters: &[(&str, &str)], picked_file: Shared<Option<File>>) {
    let title = title.to_string();
    let filters = filters
        .iter()
        .map(|(name, extension)| (name.to_string(), extension.to_string()))
        .collect::<Vec<_>>();

    wasm_bindgen_futures::spawn_local(async move {
        let mut dialog = rfd::AsyncFileDialog::new().set_title(title);

        for (filter_name, extension) in filters.into_iter() {
            dialog = dialog.add_filter(filter_name, &[extension]);
        }

        if let Some(file) = dialog.pick_file().await {
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
pub fn pick_multiple_files(title: &str, filters: &[(&str, &str)], picked_files: Shared<Vec<File>>) {
    use std::thread::spawn;

    let title = title.to_string();
    let filters = filters
        .iter()
        .map(|(name, extension)| (name.to_string(), extension.to_string()))
        .collect::<Vec<_>>();

    spawn(move || {
        let mut dialog = rfd::FileDialog::new().set_title(title);

        for (filter_name, extension) in filters.into_iter() {
            dialog = dialog.add_filter(filter_name, &[extension]);
        }

        if let Some(files) = dialog.pick_files() {
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
pub fn pick_multiple_files(title: &str, filters: &[(&str, &str)], picked_files: Shared<Vec<File>>) {
    let title = title.to_string();
    let filters = filters
        .iter()
        .map(|(name, extension)| (name.to_string(), extension.to_string()))
        .collect::<Vec<_>>();

    wasm_bindgen_futures::spawn_local(async move {
        let mut dialog = rfd::FileDialog::new().set_title(title);

        for (filter_name, extension) in filters.into_iter() {
            dialog = dialog.add_filter(filter_name, &[extension]);
        }

        if let Some(files) = dialog.pick_files().await {
            let files = files
                .into_iter()
                .map(|file| File {
                    path_buf: file.file_name().into(),
                    image: file.read().await,
                })
                .collect::<Vec<_>>();
            picked_files.with_mut(|f| {
                *f = files;
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
