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
                // if the header exists...
                if let Some(header) = req.supergraph_request.headers().get("authorization") {
                    // ... we'll try to decode it...
                    if let Ok(decode_result) = base64::decode(header) {
                        // ... and finally try to convert the returned vec<u8> to a string for the header
                        if let Ok(decoded_str_value) = String::from_utf8(decode_result) {
                            req.subgraph_request.headers_mut().append(
                                "Authorization64Decoded",
                                decoded_str_value.parse().expect("error adding header"),
                            );
                        }
                    }
                }
                req
            })
            .service(service)
            .boxed()
    }
}

register_plugin!("poc", "base64_decode_header", Base64EncodeHeader);
