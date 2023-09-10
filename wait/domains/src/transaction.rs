pub trait Transaction {
    fn execute(&mut self) -> ();
}