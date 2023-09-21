use crate::errors::ErrorDomain;
pub trait Transaction<T>{
    fn execute(&mut self) -> Result<T, Box<dyn ErrorDomain>>;
}