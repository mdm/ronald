use std::cell::RefCell;
use std::rc::Rc;

pub type TapeControllerShared = Rc<RefCell<TapeController>>;

pub struct TapeController {}

impl TapeController {
    pub fn new_shared() -> TapeControllerShared {
        let tape = TapeController {};

        Rc::new(RefCell::new(tape))
    }

    pub fn switch_motor(&mut self, on: bool) {
        () // TODO: implement this
    }

    pub fn read_sample(&self) -> bool {
        false // TODO: implement this
    }

    pub fn write_sample(&mut self, high_amplitude: bool) {
        () // TODO: implement this
    }
}
