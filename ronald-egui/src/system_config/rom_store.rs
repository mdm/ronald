use std::cell::RefCell;
use std::rc::Rc;

use idb::{Database, DatabaseEvent, Error, Factory, ObjectStoreParams, TransactionMode};
use js_sys::{Reflect, Uint8Array};
use wasm_bindgen::JsValue;

const DB_NAME: &str = "ronald";
const DB_VERSION: u32 = 1;
const STORE_ROMS: &str = "system_roms";

thread_local! {
    /// The open connection, reused across scans. `idb::Database` is not `Clone`, hence the
    /// `Rc`. This is a connection handle, not application state.
    static DATABASE: RefCell<Option<Rc<Database>>> = const { RefCell::new(None) };
}

/// A complete record from the `system_roms` store.
pub struct StoredRom {
    pub hash: Vec<u8>,
    pub name: String,
    pub image: Vec<u8>,
}

impl std::fmt::Debug for StoredRom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Summarize the image rather than dumping 16KB of bytes.
        f.debug_struct("StoredRom")
            .field("hash", &hex::encode(&self.hash))
            .field("name", &self.name)
            .field("image", &format_args!("{} bytes", self.image.len()))
            .finish()
    }
}

async fn open_database() -> Result<Rc<Database>, Error> {
    if let Some(database) = DATABASE.with_borrow(|database| database.clone()) {
        return Ok(database);
    }

    let factory = Factory::new()?;
    let mut open_request = factory.open(DB_NAME, Some(DB_VERSION))?;
    open_request.on_upgrade_needed(|event| {
        let Ok(database) = event.database() else {
            log::error!("Upgrade event without database");
            return;
        };

        // No key path and no auto-increment, so keys are out-of-line and supplied on write.
        if !database.store_names().iter().any(|name| name == STORE_ROMS)
            && let Err(e) = database.create_object_store(STORE_ROMS, ObjectStoreParams::new())
        {
            log::error!("Failed to create object store {STORE_ROMS}: {e}");
        }
    });

    let database = Rc::new(open_request.await?);
    DATABASE.with_borrow_mut(|cached| *cached = Some(database.clone()));

    Ok(database)
}

/// Loads every stored ROM, image included.
pub async fn load_roms() -> Result<Vec<StoredRom>, Error> {
    let database = open_database().await?;
    let transaction = database.transaction(&[STORE_ROMS], TransactionMode::ReadOnly)?;
    let store = transaction.object_store(STORE_ROMS)?;

    // Keys are out-of-line, so the hashes have to be fetched separately. getAll and
    // getAllKeys both iterate in ascending key order, so the two vectors line up index
    // for index.
    let keys = store.get_all_keys(None, None)?.await?;
    let values = store.get_all(None, None)?.await?;
    transaction.await?;

    Ok(keys
        .into_iter()
        .zip(values)
        .filter_map(to_stored_rom)
        .collect())
}

fn to_stored_rom((key, value): (JsValue, JsValue)) -> Option<StoredRom> {
    let name = Reflect::get(&value, &JsValue::from_str("name"))
        .ok()?
        .as_string()?;
    let image = Reflect::get(&value, &JsValue::from_str("image")).ok()?;

    Some(StoredRom {
        hash: Uint8Array::new(&key).to_vec(),
        name,
        image: Uint8Array::new(&image).to_vec(),
    })
}
