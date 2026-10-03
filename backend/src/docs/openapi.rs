use utoipa::{
    Modify, OpenApi,
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Tsundoku API",
        description = "Backend for Tsundoku: save, organize, and share the links you find."
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "Health", description = "Service status"),
        (name = "Sections", description = "Top-level named sections of the Collections area"),
        (name = "Categories", description = "Categories and nested subcategories inside a section"),
        (name = "Resources", description = "Saved links inside a category, with visited and shareable flags"),
        (name = "Platforms", description = "Social Links: platforms such as YouTube or TikTok"),
        (name = "Accounts", description = "Social Links: the user's accounts on a platform"),
    )
)]
pub struct ApiDoc;

// Adds the "Authorize" option to the docs
struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert_with(Default::default);
        components.add_security_scheme(
            "dev_user",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::with_description(
                "x-user-id",
                "DEV ONLY: paste a user UUID. Replaced by Google sign-in and JWT later.",
            ))),
        );
    }
}
