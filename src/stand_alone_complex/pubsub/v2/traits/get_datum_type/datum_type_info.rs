use super::DatumType;

pub (super) trait DatumTypeInfo<T> {
    fn datum_type_info() -> DatumType;
}
