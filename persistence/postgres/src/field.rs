use std::fmt::Debug;

use migration::{Nullable, ValueType, ValueTypeErr};
use sea_orm::{DbErr, TryFromU64, TryGetError, TryGetable, Value, prelude::Uuid};
use time::{Date, Duration, PrimitiveDateTime, UtcDateTime};
use todolist_core::model::{TodoId, TodoStatus, TodoTitle};

#[derive(Eq, PartialEq, PartialOrd, Clone, Debug)]
pub struct F<T: Debug>(pub T);

impl<T> F<T>
where
    T: Debug,
{
    pub fn from(value: &T) -> Self
    where
        T: Clone,
    {
        Self(value.clone())
    }
}

#[derive(Eq, PartialEq, PartialOrd, Clone, Debug)]
pub enum FO<T: Debug> {
    Some(F<T>),
    None,
}

impl From<F<TodoId>> for Value {
    fn from(value: F<TodoId>) -> Self {
        Self::Uuid(Some(value.0.0))
    }
}

impl From<F<TodoTitle>> for Value {
    fn from(value: F<TodoTitle>) -> Self {
        Self::String(Some(value.0.0))
    }
}

impl From<F<Date>> for Value {
    fn from(value: F<Date>) -> Self {
        Self::TimeDate(Some(value.0))
    }
}

impl From<F<TodoStatus>> for Value {
    fn from(value: F<TodoStatus>) -> Self {
        Self::Int(Some(value.0 as i32))
    }
}

impl From<F<UtcDateTime>> for Value {
    fn from(value: F<UtcDateTime>) -> Self {
        Self::TimeDateTime(Some(PrimitiveDateTime::new(value.0.date(), value.0.time())))
    }
}

impl From<F<Duration>> for Value {
    fn from(value: F<Duration>) -> Self {
        Self::Int(Some(value.0.whole_seconds() as i32))
    }
}

impl From<F<String>> for Value {
    fn from(value: F<String>) -> Self {
        Self::String(Some(value.0))
    }
}

impl<T> From<FO<T>> for Value
where
    Value: From<F<T>>,
    F<T>: Nullable,
    T: Debug,
{
    fn from(value: FO<T>) -> Self {
        match value {
            FO::Some(value) => Value::from(value),
            FO::None => <F<T> as Nullable>::null(),
        }
    }
}

impl Nullable for F<Date> {
    fn null() -> Value {
        Value::TimeDate(None)
    }
}

impl Nullable for F<UtcDateTime> {
    fn null() -> Value {
        Value::TimeDateTime(None)
    }
}

impl Nullable for F<Duration> {
    fn null() -> Value {
        Value::Int(None)
    }
}

impl Nullable for F<String> {
    fn null() -> Value {
        Value::String(None)
    }
}

impl ValueType for F<TodoId> {
    fn try_from(v: Value) -> Result<Self, migration::ValueTypeErr> {
        match v {
            Value::Uuid(Some(value)) => Ok(F(TodoId(value))),
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        <Uuid as ValueType>::type_name()
    }

    fn array_type() -> migration::ArrayType {
        <Uuid as ValueType>::array_type()
    }

    fn column_type() -> sea_orm::ColumnType {
        <Uuid as ValueType>::column_type()
    }
}

impl ValueType for F<TodoTitle> {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::String(Some(value)) => Ok(F(TodoTitle(value))),
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        <String as ValueType>::type_name()
    }

    fn array_type() -> migration::ArrayType {
        <String as ValueType>::array_type()
    }

    fn column_type() -> sea_orm::ColumnType {
        <String as ValueType>::column_type()
    }
}

impl ValueType for FO<Date> {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::TimeDate(Some(value)) => Ok(FO::Some(F(value))),
            Value::TimeTime(None) => Ok(FO::None),
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        <Date as ValueType>::type_name()
    }

    fn array_type() -> migration::ArrayType {
        <Date as ValueType>::array_type()
    }

    fn column_type() -> sea_orm::ColumnType {
        <Date as ValueType>::column_type()
    }
}

impl ValueType for FO<UtcDateTime> {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::TimeDateTime(Some(value)) => {
                Ok(FO::Some(F(UtcDateTime::new(value.date(), value.time()))))
            }
            Value::TimeDateTime(None) => Ok(FO::None),
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        <PrimitiveDateTime as ValueType>::type_name()
    }

    fn array_type() -> migration::ArrayType {
        <PrimitiveDateTime as ValueType>::array_type()
    }

    fn column_type() -> sea_orm::ColumnType {
        <PrimitiveDateTime as ValueType>::column_type()
    }
}

impl ValueType for FO<Duration> {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::Int(Some(value)) => Ok(FO::Some(F(Duration::seconds(value.into())))),
            Value::Int(None) => Ok(FO::None),
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        <i32 as ValueType>::type_name()
    }

    fn array_type() -> migration::ArrayType {
        <i32 as ValueType>::array_type()
    }

    fn column_type() -> sea_orm::ColumnType {
        <i32 as ValueType>::column_type()
    }
}

impl ValueType for F<TodoStatus> {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::Int(Some(value)) => match TodoStatus::try_from(value) {
                Ok(value) => Ok(F(value)),
                _ => Err(ValueTypeErr),
            },
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        <i32 as ValueType>::type_name()
    }

    fn array_type() -> migration::ArrayType {
        <i32 as ValueType>::array_type()
    }

    fn column_type() -> sea_orm::ColumnType {
        <i32 as ValueType>::column_type()
    }
}

impl ValueType for FO<String> {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::String(Some(value)) => Ok(FO::Some(F(value))),
            Value::String(None) => Ok(FO::None),
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        <String as ValueType>::type_name()
    }

    fn array_type() -> migration::ArrayType {
        <String as ValueType>::array_type()
    }

    fn column_type() -> sea_orm::ColumnType {
        <String as ValueType>::column_type()
    }
}

impl TryGetable for F<TodoId> {
    fn try_get_by<I: sea_orm::ColIdx>(
        res: &sea_orm::QueryResult,
        index: I,
    ) -> Result<Self, TryGetError> {
        let value: Uuid = res.try_get_by(index)?;
        Ok(F(TodoId(value)))
    }
}

impl TryGetable for F<TodoTitle> {
    fn try_get_by<I: sea_orm::ColIdx>(
        res: &sea_orm::QueryResult,
        index: I,
    ) -> Result<Self, TryGetError> {
        let value: String = res.try_get_by(index)?;
        Ok(F(TodoTitle(value)))
    }
}

impl TryGetable for F<Date> {
    fn try_get_by<I: sea_orm::ColIdx>(
        res: &sea_orm::QueryResult,
        index: I,
    ) -> Result<Self, TryGetError> {
        let value: Date = res.try_get_by(index)?;
        Ok(F(value))
    }
}

impl TryGetable for F<UtcDateTime> {
    fn try_get_by<I: sea_orm::ColIdx>(
        res: &sea_orm::QueryResult,
        index: I,
    ) -> Result<Self, TryGetError> {
        let value: PrimitiveDateTime = res.try_get_by(index)?;
        Ok(F(UtcDateTime::new(value.date(), value.time())))
    }
}

impl TryGetable for F<Duration> {
    fn try_get_by<I: sea_orm::ColIdx>(
        res: &sea_orm::QueryResult,
        index: I,
    ) -> Result<Self, TryGetError> {
        let value: i32 = res.try_get_by(index)?;
        Ok(F(Duration::seconds(value as i64)))
    }
}

impl TryGetable for F<TodoStatus> {
    fn try_get_by<I: sea_orm::ColIdx>(
        res: &sea_orm::QueryResult,
        index: I,
    ) -> Result<Self, TryGetError> {
        let value: i32 = res.try_get_by(index)?;
        match TodoStatus::try_from(value) {
            Ok(value) => Ok(F(value)),
            Err(_) => Err(TryGetError::DbErr(DbErr::Type("Invalid TodoStatus".into()))),
        }
    }
}

impl TryGetable for F<String> {
    fn try_get_by<I: sea_orm::ColIdx>(
        res: &sea_orm::QueryResult,
        index: I,
    ) -> Result<Self, TryGetError> {
        let value: String = res.try_get_by(index)?;
        Ok(F(value))
    }
}

impl TryFromU64 for F<TodoId> {
    fn try_from_u64(n: u64) -> Result<Self, DbErr> {
        Ok(F(TodoId(Uuid::from_u64_pair(n, 0))))
    }
}

impl TryGetable for FO<Date> {
    fn try_get_by<I: sea_orm::ColIdx>(
        res: &sea_orm::QueryResult,
        index: I,
    ) -> Result<Self, TryGetError> {
        let result: Result<Option<Date>, DbErr> = res.try_get_by(index);
        let value = result?;
        match value {
            Some(value) => Ok(FO::Some(F(value))),
            None => Ok(FO::None),
        }
    }
}

impl TryGetable for FO<UtcDateTime> {
    fn try_get_by<I: sea_orm::ColIdx>(
        res: &sea_orm::QueryResult,
        index: I,
    ) -> Result<Self, TryGetError> {
        let value: Option<PrimitiveDateTime> = res.try_get_by(index)?;
        match value {
            Some(value) => Ok(FO::Some(F(UtcDateTime::new(value.date(), value.time())))),
            None => Ok(FO::None),
        }
    }
}

impl TryGetable for FO<Duration> {
    fn try_get_by<I: sea_orm::ColIdx>(
        res: &sea_orm::QueryResult,
        index: I,
    ) -> Result<Self, TryGetError> {
        let value: Option<i32> = res.try_get_by(index)?;
        match value {
            Some(value) => Ok(FO::Some(F(Duration::seconds(value.into())))),
            None => Ok(FO::None),
        }
    }
}

impl TryGetable for FO<String> {
    fn try_get_by<I: sea_orm::ColIdx>(
        res: &sea_orm::QueryResult,
        index: I,
    ) -> Result<Self, TryGetError> {
        let value: Option<String> = res.try_get_by(index)?;
        match value {
            Some(value) => Ok(FO::Some(F(value))),
            None => Ok(FO::None),
        }
    }
}
