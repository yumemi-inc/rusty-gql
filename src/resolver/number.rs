use serde_json::Number;

use crate::{
    FieldContext, FieldResolver, GqlValue, ResolverResult, SelectionSetContext,
    SelectionSetResolver,
};

#[async_trait::async_trait]
impl FieldResolver for i8 {
    async fn resolve_field(&self, _ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        Ok(Some(GqlValue::Number(Number::from(*self))))
    }
}

#[async_trait::async_trait]
impl SelectionSetResolver for i8 {
    async fn resolve_selection_set(
        &self,
        _ctx: &SelectionSetContext<'_>,
    ) -> ResolverResult<GqlValue> {
        Ok(GqlValue::Number(Number::from(*self)))
    }
}

#[async_trait::async_trait]
impl FieldResolver for i16 {
    async fn resolve_field(&self, _ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        Ok(Some(GqlValue::Number(Number::from(*self))))
    }
}

#[async_trait::async_trait]
impl SelectionSetResolver for i16 {
    async fn resolve_selection_set(
        &self,
        _ctx: &SelectionSetContext<'_>,
    ) -> ResolverResult<GqlValue> {
        Ok(GqlValue::Number(Number::from(*self)))
    }
}

#[async_trait::async_trait]
impl FieldResolver for i32 {
    async fn resolve_field(&self, _ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        Ok(Some(GqlValue::Number(Number::from(*self))))
    }
}

#[async_trait::async_trait]
impl SelectionSetResolver for i32 {
    async fn resolve_selection_set(
        &self,
        _ctx: &SelectionSetContext<'_>,
    ) -> ResolverResult<GqlValue> {
        Ok(GqlValue::Number(Number::from(*self)))
    }
}

#[async_trait::async_trait]
impl FieldResolver for i64 {
    async fn resolve_field(&self, _ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        Ok(Some(GqlValue::Number(Number::from(*self))))
    }
}

#[async_trait::async_trait]
impl SelectionSetResolver for i64 {
    async fn resolve_selection_set(
        &self,
        _ctx: &SelectionSetContext<'_>,
    ) -> ResolverResult<GqlValue> {
        Ok(GqlValue::Number(Number::from(*self)))
    }
}

#[async_trait::async_trait]
impl FieldResolver for u8 {
    async fn resolve_field(&self, _ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        Ok(Some(GqlValue::Number(Number::from(*self))))
    }
}

#[async_trait::async_trait]
impl SelectionSetResolver for u8 {
    async fn resolve_selection_set(
        &self,
        _ctx: &SelectionSetContext<'_>,
    ) -> ResolverResult<GqlValue> {
        Ok(GqlValue::Number(Number::from(*self)))
    }
}
#[async_trait::async_trait]
impl FieldResolver for u16 {
    async fn resolve_field(&self, _ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        Ok(Some(GqlValue::Number(Number::from(*self))))
    }
}

#[async_trait::async_trait]
impl SelectionSetResolver for u16 {
    async fn resolve_selection_set(
        &self,
        _ctx: &SelectionSetContext<'_>,
    ) -> ResolverResult<GqlValue> {
        Ok(GqlValue::Number(Number::from(*self)))
    }
}

#[async_trait::async_trait]
impl FieldResolver for u32 {
    async fn resolve_field(&self, _ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        Ok(Some(GqlValue::Number(Number::from(*self))))
    }
}

#[async_trait::async_trait]
impl SelectionSetResolver for u32 {
    async fn resolve_selection_set(
        &self,
        _ctx: &SelectionSetContext<'_>,
    ) -> ResolverResult<GqlValue> {
        Ok(GqlValue::Number(Number::from(*self)))
    }
}

#[async_trait::async_trait]
impl FieldResolver for u64 {
    async fn resolve_field(&self, _ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        Ok(Some(GqlValue::Number(Number::from(*self))))
    }
}

#[async_trait::async_trait]
impl SelectionSetResolver for u64 {
    async fn resolve_selection_set(
        &self,
        _ctx: &SelectionSetContext<'_>,
    ) -> ResolverResult<GqlValue> {
        Ok(GqlValue::Number(Number::from(*self)))
    }
}

#[async_trait::async_trait]
impl FieldResolver for usize {
    async fn resolve_field(&self, _ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        Ok(Some(GqlValue::Number(Number::from(*self))))
    }
}

#[async_trait::async_trait]
impl SelectionSetResolver for usize {
    async fn resolve_selection_set(
        &self,
        _ctx: &SelectionSetContext<'_>,
    ) -> ResolverResult<GqlValue> {
        Ok(GqlValue::Number(Number::from(*self)))
    }
}

#[async_trait::async_trait]
impl FieldResolver for isize {
    async fn resolve_field(&self, _ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        Ok(Some(GqlValue::Number(Number::from(*self))))
    }
}

#[async_trait::async_trait]
impl SelectionSetResolver for isize {
    async fn resolve_selection_set(
        &self,
        _ctx: &SelectionSetContext<'_>,
    ) -> ResolverResult<GqlValue> {
        Ok(GqlValue::Number(Number::from(*self)))
    }
}

#[async_trait::async_trait]
impl FieldResolver for f32 {
    async fn resolve_field(&self, _ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        match Number::from_f64(*self as f64) {
            Some(n) => Ok(Some(GqlValue::Number(n))),
            None => Ok(Some(GqlValue::Null)),
        }
    }
}

#[async_trait::async_trait]
impl SelectionSetResolver for f32 {
    async fn resolve_selection_set(
        &self,
        _ctx: &SelectionSetContext<'_>,
    ) -> ResolverResult<GqlValue> {
        match Number::from_f64(*self as f64) {
            Some(n) => Ok(GqlValue::Number(n)),
            None => Ok(GqlValue::Null),
        }
    }
}

#[async_trait::async_trait]
impl FieldResolver for f64 {
    async fn resolve_field(&self, _ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        match Number::from_f64(*self) {
            Some(n) => Ok(Some(GqlValue::Number(n))),
            None => Ok(Some(GqlValue::Null)),
        }
    }
}

#[async_trait::async_trait]
impl SelectionSetResolver for f64 {
    async fn resolve_selection_set(
        &self,
        _ctx: &SelectionSetContext<'_>,
    ) -> ResolverResult<GqlValue> {
        match Number::from_f64(*self) {
            Some(n) => Ok(GqlValue::Number(n)),
            None => Ok(GqlValue::Null),
        }
    }
}
