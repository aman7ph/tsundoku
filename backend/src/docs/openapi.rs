use utoipa::{
    Modify, OpenApi,
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
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
        (name = "Auth", description = "Sign in with Google, refresh and end sessions"),
        (name = "Sections", description = "Collections: top-level named sections"),
        (name = "Categories", description = "Collections: categories and nested subcategories inside a section"),
        (name = "Resources", description = "Collections: saved links inside a category, with visited and shareable flags"),
        (name = "Platforms", description = "Social Links: platforms such as YouTube or TikTok"),
        (name = "Accounts", description = "Social Links: the user's accounts on a platform"),
        (name = "Social Categories", description = "Social Links: categories inside an account"),
        (name = "Social Links", description = "Social Links: saved links inside a category, with a shareable flag"),
    )
)]
pub struct ApiDoc;

// Adds the "Authorize" option to the docs
struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert_with(Default::default);
        components.add_security_scheme(
            "bearer",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .description(Some(
                        "Paste the access_token from /auth/google or /auth/dev-login",
                    ))
                    .build(),
            ),
        );
    }
}
