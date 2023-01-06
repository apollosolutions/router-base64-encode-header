// This is a bare-bones plugin that you can duplicate when creating your own.
use apollo_router::plugin::Plugin;
use apollo_router::plugin::PluginInit;
use apollo_router::register_plugin;
use apollo_router::services::*;
use schemars::JsonSchema;
use serde::Deserialize;
use tower::{BoxError, ServiceBuilder, ServiceExt};

struct Base64EncodeHeader {
    #[allow(dead_code)]
    configuration: Conf,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
struct Conf {}

#[async_trait::async_trait]
impl Plugin for Base64EncodeHeader {
    type Config = Conf;

    async fn new(init: PluginInit<Self::Config>) -> Result<Self, BoxError> {
        Ok(Base64EncodeHeader {
            configuration: init.config,
        })
    }

    fn subgraph_service(&self, _name: &str, service: subgraph::BoxService) -> subgraph::BoxService {
        ServiceBuilder::new()
            .map_request(move |mut req: subgraph::Request| {
                if let Some(header) = req.supergraph_request.headers().get("authorization") {
                    req.subgraph_request.headers_mut().append(
                        "Authorization64Encoded",
                        base64::encode(header)
                            .parse()
                            .expect("Failed to base64 encode"),
                    );
                }
                req
            })
            .service(service)
            .boxed()
    }
}

register_plugin!("poc", "base64_encode_header", Base64EncodeHeader);
