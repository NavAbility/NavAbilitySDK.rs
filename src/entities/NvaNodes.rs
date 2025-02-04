
use uuid::Uuid;
use std::marker;


#[derive(Debug, Clone)]
pub struct Org {
    id: Uuid,
    label: String,
    description: String,
}

#[derive(Debug, Clone)]
pub struct Model {}

#[derive(Debug, Clone)]
pub struct Factorgraph {}

#[derive(Clone,Debug)]
pub struct NvaNode<'a, T: 'a> {
    pub namespace: Uuid,
    pub label: String,
    pub _marker: marker::PhantomData<&'a T>
    // https://doc.rust-lang.org/nomicon/phantom-data.html
}
