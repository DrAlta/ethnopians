use super::Sting;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DatumType {
    I8,
    String,
}


impl DatumType {
    #[allow(private_bounds)]
    pub fn get<T>() -> DatumType where DatumType: Foo<T> {
        <DatumType as Foo<T>>::foo()
    }
}

trait Foo<T>{
    fn foo() -> DatumType;
}

impl Foo<i8> for DatumType{
    fn foo() -> DatumType {
        DatumType::I8
    }
}
impl Foo<Sting> for DatumType{
    fn foo() -> DatumType {
        DatumType::String
    }
}




pub trait GetDatumType<const INDEX : usize>{
    fn get_data_type(&self) -> Option<DatumType>;
}


impl GetDatumType<0> for Vec<(i8, i8)> {
    fn get_data_type(&self) -> Option<DatumType> {
        Some(DatumType::I8)
    }
}
impl GetDatumType<1> for Vec<(i8, i8)> {
    fn get_data_type(&self) -> Option<DatumType> {
        Some(DatumType::I8)
    }
}


impl GetDatumType<0> for Vec<(i8, Sting)> {
    fn get_data_type(&self) -> Option<DatumType> {
        Some(DatumType::I8)
    }
}
impl GetDatumType<1> for Vec<(i8, Sting)> {
    fn get_data_type(&self) -> Option<DatumType> {
        Some(DatumType::String)
    }
}


impl GetDatumType<0> for Vec<(Sting, Sting)> {
    fn get_data_type(&self) -> Option<DatumType> {
        Some(DatumType::String)
    }
}
impl GetDatumType<1> for Vec<(Sting, Sting)> {
    fn get_data_type(&self) -> Option<DatumType> {
        Some(DatumType::String)
    }
}