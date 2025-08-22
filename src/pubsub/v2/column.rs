use std::sync::LazyLock;
use super::Sting;
#[derive(Debug)]
pub enum Column {
    I8(Vec<i8>),
    String(Vec<Sting>),
}

impl Column {
    pub fn empty_i8() -> &'static Self {
        &EMPTY_I8_COLUMN
    }
    pub fn empty_string() -> &'static Self {
        &EMPTY_STRING_COLUMN
    }
}

static EMPTY_I8: LazyLock<Vec<i8>> = LazyLock::new(|| {
    Vec::new()
});

pub fn empty_i8() -> &'static Vec<i8> {
    &EMPTY_I8  
}

static EMPTY_STRING: LazyLock<Vec<Sting>> = LazyLock::new(|| {
    Vec::new()
});

pub fn empty_string() -> &'static Vec<Sting> {
    &EMPTY_STRING  
}

static EMPTY_I8_COLUMN: LazyLock<Column> = LazyLock::new(|| {
    Column::I8(Vec::new())
});

static EMPTY_STRING_COLUMN: LazyLock<Column> = LazyLock::new(|| {
    Column::String(Vec::new())
});
