/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Test<Args> {
    pub _address: u8,
    pub _phantom_0: ::std::marker::PhantomData<Args>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Outer<T> {
    pub _address: u8,
    pub _phantom_0: ::std::marker::PhantomData<T>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Outer_Inner<T> {
    pub _address: u8,
    pub _phantom_0: ::std::marker::PhantomData<T>,
}