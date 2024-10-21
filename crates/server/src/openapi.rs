use crate::handlers::{assets::*, auth::*, users::*};
use chrono::{SecondsFormat, Utc};
use database::models::*;
use shared::constants::Asset;
use utoipa::{
    openapi::{
        self,
        schema::SchemaType,
        security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    },
    Modify, OpenApi, ToSchema,
};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "XippleFi OpenApi",
        version = "1.0.0",
        description = "🦀 The small, blazing fast, high performance XippleFi http server",
        contact (name = "goni098", url = "https://github.com/goni098")
    ),
    paths(
        get_asset,
        get_assets,
        get_message,
        verify_signature,
        get_user_assets,
    ),
    components(
        schemas(
            // query
            // body
            VerifySignaturePayload,
            // responses
            AssetModel,
            GetAssetResponse,
            GetAssetsResponse,
            GetMessageResponse,
            VerifySignatureResponse,
            // custom types
            DateTimeWithTimeZone,
            Decimal,
            // enums
            Asset
        ),
        responses()
    ),
    modifiers(&BearerSecurity)
  )]
pub struct ApiDoc;

struct BearerSecurity;
struct DateTimeWithTimeZone;
struct Decimal;

impl Modify for BearerSecurity {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "BearerAuth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}

impl<'__s> ToSchema<'__s> for DateTimeWithTimeZone {
    fn schema() -> (&'__s str, openapi::RefOr<openapi::schema::Schema>) {
        (
            "DateTimeWithTimeZone",
            openapi::ObjectBuilder::new()
                .schema_type(SchemaType::String)
                .description(Some("RFC 3339, ISO 8601 date and time string"))
                .default(Some(
                    Utc::now()
                        .to_rfc3339_opts(SecondsFormat::Millis, true)
                        .into(),
                ))
                .into(),
        )
    }
}

impl<'__s> ToSchema<'__s> for Decimal {
    fn schema() -> (&'__s str, openapi::RefOr<openapi::schema::Schema>) {
        (
            "Decimal",
            openapi::ObjectBuilder::new()
                .schema_type(SchemaType::String)
                .description(Some("Decimal value"))
                .default(Some(serde_json::Value::String("9000".to_string())))
                .into(),
        )
    }
}
