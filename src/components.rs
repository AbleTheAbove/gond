use serde::Serialize;

pub type Id = u16;
pub type WorldId = u16;
pub type NationId = u16;
pub type RegionId = u16;
pub type ProvinceId = u16;
pub type PopGroupId = u16;
pub type ResourceId = u16;

#[derive(Debug, Serialize)]
pub struct Name(pub &'static str);

#[derive(Debug, Serialize)]
pub struct Money(pub u32);

#[derive(Debug, Serialize)]
pub struct Quantity(pub u16);
