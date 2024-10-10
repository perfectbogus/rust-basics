mod copy;
mod cut;
mod paste;

pub use copy::CopyCmd;
pub use cut::CutCmd;
pub use paste::PasteCmd;

pub trait Command {
    fn execute(&mut self, app: &mut cursive::Cursive) -> bool;
    fn undo(&mut self, app: &mut cursive::Cursive);
}

