use crate::errors::ErrorDomain;
pub trait Transaction {
    fn execute(&mut self) -> Result<(), Box<dyn ErrorDomain>>;
}