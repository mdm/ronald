pub trait SharedExt<T> {
    fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R;
    fn try_with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> Option<R>;
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use super::SharedExt;
    use std::{cell::RefCell, rc::Rc};

    pub type Shared<T> = Rc<RefCell<T>>;

    pub fn shared<T>(value: T) -> Shared<T> {
        Rc::new(RefCell::new(value))
    }

    impl<T> SharedExt<T> for Shared<T> {
        fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
            f(&mut *self.borrow_mut())
        }

        fn try_with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> Option<R> {
            self.try_borrow_mut().ok().map(|mut guard| f(&mut *guard))
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use super::SharedExt;
    use std::sync::{Arc, Mutex};

    pub type Shared<T> = Arc<Mutex<T>>;

    pub fn shared<T>(value: T) -> Shared<T> {
        Arc::new(Mutex::new(value))
    }

    impl<T> SharedExt<T> for Shared<T> {
        fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
            f(&mut *self.lock().unwrap())
        }

        fn try_with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> Option<R> {
            self.try_lock().ok().map(|mut guard| f(&mut *guard))
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm::*;

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;
