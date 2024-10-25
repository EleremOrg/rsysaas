use stefn::ErrorMessage;

use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};

use super::{ingestion, recommendation};

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    security(("jwt" = ["*"])),
    servers(
        (url = "{protocol}://{domain}:{port}/api/{version}", description = "Local server",
            variables(
                ("protocol" = (default = "http", enum_values("http", "https"), description = "Protocl used to request")),
                ("domain" = (default = "127.0.0.1", enum_values("127.0.0.1", "elerem.com"), description = "Default domain for API")),
                ("port" = (default = "8001", enum_values("8000", "8001", "8002"), description = "Supported ports for API")),
                ("version" = (default = "v1", enum_values("v1"), description = "Supported versions for API")),
            )
        )),
    nest(
        (path = "/", api = recommendation::ApiDoc, tags = ["Recommendations"]),
        (path = "/", api = ingestion::ApiDoc),
    ),
    components(schemas(ErrorMessage), responses(ErrorMessage)),
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "jwt",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            )
        }
    }
}
