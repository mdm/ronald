use std::cell::RefCell;
use std::rc::Rc;

use idb::{Database, DatabaseEvent, Error, Factory, ObjectStoreParams, TransactionMode};
use js_sys::{Object, Reflect, Uint8Array};
use wasm_bindgen::JsValue;

const DB_NAME: &str = "ronald";
const DB_VERSION: u32 = 1;
const STORE_ROMS: &str = "system_roms";

thread_local! {
    static DATABASE: RefCell<Option<Rc<Database>>> = const { RefCell::new(None) };
}

pub struct StoredRom {
    pub hash: Vec<u8>,
    pub name: String,
    pub image: Vec<u8>,
}

impl From<&StoredRom> for JsValue {
    fn from(rom: &StoredRom) -> Self {
        let value = Object::new();
        let image = Uint8Array::from(rom.image.as_slice());

        Reflect::set(
            &value,
            &JsValue::from_str("name"),
            &JsValue::from_str(&rom.name),
        )
        .expect("failed to set ROM name");
        Reflect::set(&value, &JsValue::from_str("image"), image.as_ref())
            .expect("failed to set ROM image");

        value.into()
    }
}

impl TryFrom<(JsValue, JsValue)> for StoredRom {
    type Error = JsValue;

    fn try_from((key, value): (JsValue, JsValue)) -> Result<Self, Self::Error> {
        let name = Reflect::get(&value, &JsValue::from_str("name"))?
            .as_string()
            .ok_or_else(|| JsValue::from_str("record has no name"))?;
        let image = Reflect::get(&value, &JsValue::from_str("image"))?;

        Ok(Self {
            hash: Uint8Array::new(&key).to_vec(),
            name,
            image: Uint8Array::new(&image).to_vec(),
        })
    }
}

impl std::fmt::Debug for StoredRom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

pub async fn load_roms() -> Result<Vec<StoredRom>, Error> {
    let database = open_database().await?;
    let transaction = database.transaction(&[STORE_ROMS], TransactionMode::ReadOnly)?;
    let store = transaction.object_store(STORE_ROMS)?;

    let keys = store.get_all_keys(None, None)?.await?;
    let values = store.get_all(None, None)?.await?;
    transaction.await?;

    Ok(keys
        .into_iter()
        .zip(values)
        .filter_map(|record| {
            StoredRom::try_from(record)
                .inspect_err(|e| log::warn!("Skipping malformed ROM record: {:?}", e))
                .ok()
        })
        .collect())
}

pub async fn store_roms(roms: &[StoredRom]) -> Result<(), Error> {
    let database = open_database().await?;
    let transaction = database.transaction(&[STORE_ROMS], TransactionMode::ReadWrite)?;
    let store = transaction.object_store(STORE_ROMS)?;

    let requests = roms
        .iter()
        .map(|rom| {
            let key = JsValue::from(Uint8Array::from(rom.hash.as_slice()));
            store.put(&JsValue::from(rom), Some(&key))
        })
        .collect::<Result<Vec<_>, _>>()?;

    for request in requests {
        request.await?;
    }
    transaction.commit()?.await?;

    Ok(())
}
