#![forbid(unsafe_code)]
use crate::{
    data::{DataType, Value},
    error::{Error, ErrorCtx, ErrorWithCtx, Result, UnexpectedTypeError},
    object::Schema,
    ObjectId,
};
use rusqlite::{types::FromSqlError, ToSql};
use std::{borrow::Cow, fmt::Write};

////////////////////////////////////////////////////////////////////////////////

pub type Row<'a> = Vec<Value<'a>>;
pub type RowSlice<'a> = [Value<'a>];

////////////////////////////////////////////////////////////////////////////////

pub(crate) trait StorageTransaction {
    fn table_exists(&self, table: &str) -> Result<bool>;
    fn create_table(&self, schema: &Schema) -> Result<()>;

    fn insert_row(&self, schema: &Schema, row: &RowSlice) -> Result<ObjectId>;
    fn update_row(&self, id: ObjectId, schema: &Schema, row: &RowSlice) -> Result<()>;
    fn select_row(&self, id: ObjectId, schema: &Schema) -> Result<Row<'static>>;
    fn delete_row(&self, id: ObjectId, schema: &Schema) -> Result<()>;

    fn commit(&self) -> Result<()>;
    fn rollback(&self) -> Result<()>;
}

impl<'a> StorageTransaction for rusqlite::Transaction<'a> {
    fn table_exists(&self, table: &str) -> Result<bool> {
        // TODO: your code goes here.
        unimplemented!()
    }

    fn create_table(&self, schema: &Schema) -> Result<()> {
        // TODO: your code goes here.
        unimplemented!()
    }

    fn insert_row(&self, schema: &Schema, row: &RowSlice) -> Result<ObjectId> {
        // TODO: your code goes here.
        unimplemented!()
    }

    fn update_row(&self, id: ObjectId, schema: &Schema, row: &RowSlice) -> Result<()> {
        // TODO: your code goes here.
        unimplemented!()
    }

    fn select_row(&self, id: ObjectId, schema: &Schema) -> Result<Row<'static>> {
        // TODO: your code goes here.
        unimplemented!()
    }

    fn delete_row(&self, id: ObjectId, schema: &Schema) -> Result<()> {
        // TODO: your code goes here.
        unimplemented!()
    }

    fn commit(&self) -> Result<()> {
        // TODO: your code goes here.
        unimplemented!()
    }

    fn rollback(&self) -> Result<()> {
        // TODO: your code goes here.
        unimplemented!()
    }
}
