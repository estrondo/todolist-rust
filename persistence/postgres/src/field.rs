use sea_orm::{
    ColumnType, DbErr, TryFromU64, TryGetError, TryGetable, Value,
    prelude::Uuid,
    sea_query::{ArrayType, Nullable, ValueType, ValueTypeErr},
};
use time::{Date, Duration, PrimitiveDateTime, UtcDateTime};
use todolist_core::model::{TodoId, TodoStatus, TodoTitle};

use crate::Field;

impl From<Field<TodoId>> for Value {
    fn from(value: Field<TodoId>) -> Self {
        Value::Uuid(Some(value.0.0))
    }
}

impl From<Field<TodoTitle>> for Value {
    fn from(value: Field<TodoTitle>) -> Self {
        Value::String(Some(value.0.0))
    }
}

impl From<Field<TodoStatus>> for Value {
    fn from(value: Field<TodoStatus>) -> Self {
        Value::TinyInt(Some(value.0 as i8))
    }
}

impl From<Field<Date>> for Value {
    fn from(value: Field<Date>) -> Self {
        Value::TimeDate(Some(value.0))
    }
}

impl Nullable for Field<Date> {
    fn null() -> Value {
        Value::TimeDate(None)
    }
}

impl Nullable for Field<UtcDateTime> {
    fn null() -> Value {
        Value::TimeDateTime(None)
    }
}

impl Nullable for Field<Duration> {
    fn null() -> Value {
        Value::Int(None)
    }
}

impl Nullable for Field<String> {
    fn null() -> Value {
        Value::String(None)
    }
}

impl From<Field<UtcDateTime>> for Value {
    fn from(value: Field<UtcDateTime>) -> Self {
        Value::TimeDateTime(Some(PrimitiveDateTime::new(value.0.date(), value.0.time())))
    }
}

impl From<Field<Duration>> for Value {
    fn from(value: Field<Duration>) -> Self {
        Value::Int(Some(value.0.whole_seconds() as i32))
    }
}

impl From<Field<String>> for Value {
    fn from(value: Field<String>) -> Self {
        Value::String(Some(value.0))
    }
}

impl TryGetable for Field<TodoId> {
    fn try_get_by<I: sea_orm::ColIdx>(
        res: &sea_orm::QueryResult,
        index: I,
    ) -> Result<Self, sea_orm::TryGetError> {
        Ok(Field(TodoId(res.try_get_by(index)?)))
    }
}

impl TryGetable for Field<TodoTitle> {
    fn try_get_by<I: sea_orm::ColIdx>(
        res: &sea_orm::QueryResult,
        index: I,
    ) -> Result<Self, sea_orm::TryGetError> {
        Ok(Field(TodoTitle(res.try_get_by(index)?)))
    }
}

impl TryGetable for Field<Date> {
    fn try_get_by<I: sea_orm::ColIdx>(
        res: &sea_orm::QueryResult,
        index: I,
    ) -> Result<Self, sea_orm::TryGetError> {
        Ok(Field(res.try_get_by(index)?))
    }
}

impl TryGetable for Field<UtcDateTime> {
    fn try_get_by<I: sea_orm::ColIdx>(
        res: &sea_orm::QueryResult,
        index: I,
    ) -> Result<Self, sea_orm::TryGetError> {
        let primitive_date: PrimitiveDateTime = res.try_get_by(index)?;
        Ok(Field(UtcDateTime::new(
            primitive_date.date(),
            primitive_date.time(),
        )))
    }
}

impl TryGetable for Field<Duration> {
    fn try_get_by<I: sea_orm::ColIdx>(
        res: &sea_orm::QueryResult,
        index: I,
    ) -> Result<Self, sea_orm::TryGetError> {
        let duration: i32 = res.try_get_by(index)?;
        Ok(Field(Duration::seconds(duration.into())))
    }
}

impl TryGetable for Field<TodoStatus> {
    fn try_get_by<I: sea_orm::ColIdx>(
        res: &sea_orm::QueryResult,
        index: I,
    ) -> Result<Self, sea_orm::TryGetError> {
        let status: i32 = res.try_get_by(index)?;
        let todo_status = TodoStatus::try_from(status).map_err(|e| {
            TryGetError::DbErr(DbErr::Type(format!("A conversion error: {}.", e.message())))
        })?;

        Ok(Field(todo_status))
    }
}

impl TryGetable for Field<String> {
    fn try_get_by<I: sea_orm::ColIdx>(
        res: &sea_orm::QueryResult,
        index: I,
    ) -> Result<Self, TryGetError> {
        Ok(Field(res.try_get_by(index)?))
    }
}

impl ValueType for Field<TodoId> {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::Uuid(Some(v)) => Ok(Field(TodoId(v))),
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        <Uuid as ValueType>::type_name()
    }

    fn array_type() -> ArrayType {
        <Uuid as ValueType>::array_type()
    }

    fn column_type() -> ColumnType {
        <Uuid as ValueType>::column_type()
    }
}

impl ValueType for Field<TodoTitle> {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::String(Some(v)) => Ok(Field(TodoTitle(v))),
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        <String as ValueType>::type_name()
    }

    fn array_type() -> ArrayType {
        <String as ValueType>::array_type()
    }

    fn column_type() -> ColumnType {
        <String as ValueType>::column_type()
    }
}

impl ValueType for Field<Date> {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::TimeDate(Some(v)) => Ok(Field(v)),
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        <Date as ValueType>::type_name()
    }

    fn array_type() -> ArrayType {
        <Date as ValueType>::array_type()
    }

    fn column_type() -> ColumnType {
        <Date as ValueType>::column_type()
    }
}

impl ValueType for Field<UtcDateTime> {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::TimeDateTime(Some(v)) => Ok(Field(UtcDateTime::new(v.date(), v.time()))),
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        <PrimitiveDateTime as ValueType>::type_name()
    }

    fn array_type() -> ArrayType {
        <PrimitiveDateTime as ValueType>::array_type()
    }

    fn column_type() -> ColumnType {
        <PrimitiveDateTime as ValueType>::column_type()
    }
}

impl ValueType for Field<Duration> {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::Int(Some(v)) => Ok(Field(Duration::seconds(v.into()))),
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        <i32 as ValueType>::type_name()
    }

    fn array_type() -> ArrayType {
        <i32 as ValueType>::array_type()
    }

    fn column_type() -> ColumnType {
        <i32 as ValueType>::column_type()
    }
}

impl ValueType for Field<TodoStatus> {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::TinyInt(Some(v)) => {
                let v: i32 = v.into();
                let status = TodoStatus::try_from(v).map_err(|_| ValueTypeErr)?;
                Ok(Field(status))
            }
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        <u8 as ValueType>::type_name()
    }

    fn array_type() -> ArrayType {
        <u8 as ValueType>::array_type()
    }

    fn column_type() -> ColumnType {
        <u8 as ValueType>::column_type()
    }
}

impl ValueType for Field<String> {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::String(Some(v)) => Ok(Field(v)),
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        <String as ValueType>::type_name()
    }

    fn array_type() -> ArrayType {
        <String as ValueType>::array_type()
    }

    fn column_type() -> ColumnType {
        <String as ValueType>::column_type()
    }
}

impl TryFromU64 for Field<TodoId> {
    fn try_from_u64(_: u64) -> Result<Self, DbErr> {
        Err(DbErr::ConvertFromU64("Unable to convert from u64."))
    }
}
