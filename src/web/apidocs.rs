use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
  info(
    title = "NeoNet",
    version = "0.1.0",
    description = "NeoNet messaging services",
    contact(
        name = "NeoNet",
        url = "https://github.com/NeoNet-app"
    ),
  ),
  tags(
    (name = "Peer", description = "Peers setup services"),
    (name = "Data", description = "Data exchanging services"),
    (name = "User", description = "User handling services "),
  )
)]
pub struct ApiDoc;
