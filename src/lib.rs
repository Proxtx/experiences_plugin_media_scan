
use std::path::PathBuf;
use std::pin::Pin;
use timeline_types::api::CompressedEvent;
use timeline_types::available_plugins::AvailablePlugins;
use server_api::render::render_image;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct SignedMedia {
    path: String,
    signature: String
}

pub struct PluginRenderer {}

impl server_api::plugin::PluginRenderer for PluginRenderer {
    async fn new() -> PluginRenderer {
        PluginRenderer {}
    }

    fn render(&self, dimensions: (i32, i32), event: &CompressedEvent) -> Pin<Box<dyn std::future::Future<Output = Result<Vec<u32>, String>> + Send>> {
        let data = event.data.clone();

        Box::pin(async move {
            let path = match serde_json::from_value::<SignedMedia>(data) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!("Unable to read CompressedEvent: {}", e))
                }
            }.path;
            render_image(dimensions, &PathBuf::from(path)).await.map(|v|v.into_vec())
        })
    }

    fn get_timeline_type(&self) -> AvailablePlugins {
        AvailablePlugins::timeline_plugin_media_scan
    }
}
