use std::collections::{BTreeSet, HashSet, LinkedList, VecDeque};
use std::convert::TryInto;
use std::hash::Hash;

use crate::GqlValue;

use super::VariableType;

fn vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

impl<T: VariableType, const N: usize> VariableType for [T; N] {
    fn from_gql_value(value: Option<GqlValue>) -> Result<Self, String> {
        match value {
            Some(value) => match value {
                GqlValue::List(list) => {
                    let mut result = Vec::new();
                    for v in list {
                        let value = T::from_gql_value(Some(v))?;
                        result.push(value)
                    }
                    Ok(vec_to_array(result))
                }
                invalid_value => Err(format!(
                    "Expected type: list, but found {}",
                    invalid_value.to_string()
                )),
            },
            None => Err("Expected type: list, but not found".to_string()),
        }
    }

    fn into_gql_value(&self) -> GqlValue {
        let values = self.into_iter().map(|v| v.into_gql_value()).collect();
        GqlValue::List(values)
    }
}

impl<T: VariableType + Eq + Hash> VariableType for HashSet<T> {
    fn from_gql_value(value: Option<GqlValue>) -> Result<Self, String> {
        match value.unwrap_or_default() {
            GqlValue::List(list) => {
                let mut result = Vec::new();
                for v in list {
                    let value = T::from_gql_value(Some(v))?;
                    result.push(value)
                }
                let hash_set: HashSet<T> = result.into_iter().collect();
                Ok(hash_set)
            }
            value => Ok({
                let mut result = Self::default();
                result.insert(T::from_gql_value(Some(value))?);
                result
            }),
        }
    }

    fn into_gql_value(&self) -> GqlValue {
        let values = self.into_iter().map(|v| v.into_gql_value()).collect();
        GqlValue::List(values)
    }
}

impl<T: VariableType + Ord> VariableType for BTreeSet<T> {
    fn from_gql_value(value: Option<GqlValue>) -> Result<Self, String> {
        match value.unwrap_or_default() {
            GqlValue::List(list) => {
                let mut result = Vec::new();
                for v in list {
                    let value = T::from_gql_value(Some(v))?;
                    result.push(value)
                }
                let tree_set: BTreeSet<T> = result.into_iter().collect();
                Ok(tree_set)
            }
            value => Ok({
                let mut result = Self::default();
                result.insert(T::from_gql_value(Some(value))?);
                result
            }),
        }
    }

    fn into_gql_value(&self) -> GqlValue {
        let values = self.into_iter().map(|v| v.into_gql_value()).collect();
        GqlValue::List(values)
    }
}

impl<T: VariableType> VariableType for LinkedList<T> {
    fn from_gql_value(value: Option<GqlValue>) -> Result<Self, String> {
        match value.unwrap_or_default() {
            GqlValue::List(list) => {
                let mut result = Vec::new();
                for v in list {
                    let value = T::from_gql_value(Some(v))?;
                    result.push(value)
                }
                let linked_list: LinkedList<T> = result.into_iter().collect();
                Ok(linked_list)
            }
            value => Ok({
                let mut result = Self::default();
                result.push_front(T::from_gql_value(Some(value))?);
                result
            }),
        }
    }

    fn into_gql_value(&self) -> GqlValue {
        let values = self.into_iter().map(|v| v.into_gql_value()).collect();
        GqlValue::List(values)
    }
}

impl<T: VariableType> VariableType for VecDeque<T> {
    fn from_gql_value(value: Option<GqlValue>) -> Result<Self, String> {
        match value.unwrap_or_default() {
            GqlValue::List(list) => {
                let mut result = Vec::new();
                for v in list {
                    let value = T::from_gql_value(Some(v))?;
                    result.push(value)
                }
                let vec_deque: VecDeque<T> = result.into_iter().collect();
                Ok(vec_deque)
            }
            value => Ok({
                let mut result = Self::default();
                result.push_back(T::from_gql_value(Some(value))?);
                result
            }),
        }
    }

    fn into_gql_value(&self) -> GqlValue {
        let values = self.into_iter().map(|v| v.into_gql_value()).collect();
        GqlValue::List(values)
    }
}
