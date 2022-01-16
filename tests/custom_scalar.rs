use rusty_gql::*;

#[tokio::test]
pub async fn test_custom_scalar() {
    struct Query;

    #[derive(Clone)]
    struct CustomScalar(String);

    #[Resolver]
    impl CustomScalar {}

    impl GqlInputType for CustomScalar {
        fn from_gql_value(value: Option<GqlValue>) -> Result<Self, String> {
            if let Some(GqlValue::String(v)) = value {
                Ok(CustomScalar(format!("Custom-{}", v)))
            } else {
                Err(format!(
                    "{}: is invalid type for Custom Scalar",
                    value.unwrap_or(GqlValue::Null).to_string()
                ))
            }
        }

        fn into_gql_value(&self) -> GqlValue {
            GqlValue::String(self.0.clone())
        }
    }

    struct SampleResponse {
        test: CustomScalar,
    }

    #[Resolver]
    impl SampleResponse {
        async fn test(&self) -> CustomScalar {
            self.test.clone()
        }
    }

    #[Resolver]
    impl Query {
        #[allow(unused)]
        async fn test_custom_scalar(&self) -> SampleResponse {
            SampleResponse {
                test: CustomScalar("Sample".to_string()),
            }
        }
    }
    let contents = schema_content("./tests/schemas/custom_scalar.graphql");

    let container = Container::new(
        &vec![contents.as_str()],
        QueryRoot { query: Query },
        EmptyMutation,
        EmptySubscription,
        Default::default(),
    )
    .unwrap();

    let query_doc = r#"{ test_custom_scalar {test } }"#;
    let req = build_test_request(query_doc, None, Default::default());
    let expected_response = r#"{"data":{"test_custom_scalar":{"test":{}}}}"#;
    check_gql_response(req, expected_response, &container).await;
}
