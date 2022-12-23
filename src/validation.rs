pub struct Context {}

pub trait Validable {
    fn is_valid(&self, context: &Context, k: Option<u32>) -> bool;
}
