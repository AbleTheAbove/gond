use serde::Serialize;

pub type Id = u16;

#[derive(Debug, Serialize)]
pub struct Name(pub &'static str);

#[derive(Debug, Serialize)]
pub struct Money(pub u32);

#[derive(Debug, Serialize)]
pub struct Quantity(pub u16);
