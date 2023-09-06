pub trait Transaction {
    fn execute(&self) -> ();
}