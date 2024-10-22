use anyhow::{Context, Result};
use os_info::get;
use plist::Value;
use ratatui::{prelude::*, widgets::*};
use segment::*;
use semver::Version;
use std::fs::File;
use std::io::BufReader;
use tracing::{debug, error, instrument};

#[derive(Default, Debug)]
pub struct UpdatesSegmentRenderer {
    info: UpdatesInfo,
}

#[derive(Debug, Default)]
pub struct UpdatesInfo {
    updates: Vec<String>,
}

impl Info for UpdatesInfo {}

#[derive(Debug, Default)]
pub struct UpdatesInfoBuilder {}

impl InfoBuilder<UpdatesInfo> for UpdatesInfoBuilder {
    #[instrument(skip(self), fields(builder_type = "UpdatesInfoBuilder"))]
    async fn build(&self) -> Result<UpdatesInfo> {
        let current_os = get();
        let current_version = Version::parse(&current_os.version().to_string())
            .context("Failed to parse current OS version")?;
        debug!("Current OS version: {}", current_version);

        let file = File::open("/Library/Updates/ProductMetadata.plist")
            .context("Failed to open ProductMetadata.plist")?;
        debug!("Successfully opened ProductMetadata.plist");

        let reader = BufReader::new(file);
        let plist: Value =
            plist::from_reader(reader).context("Failed to parse ProductMetadata.plist")?;
        debug!("Successfully parsed ProductMetadata.plist");

        let updates = match plist {
            Value::Array(updates) => updates
                .iter()
                .filter_map(|update| {
                    if let Some(dict) = update.as_dictionary() {
                        let has_install_assistant = dict
                            .get("tags")
                            .and_then(|t| t.as_array())
                            .map(|tags| {
                                tags.iter().any(|tag| {
                                    tag.as_string()
                                        .map(|s| s.contains("SUBUNDLE:com.apple.InstallAssistant"))
                                        .unwrap_or(false)
                                })
                            })
                            .unwrap_or(false);

                        if has_install_assistant {
                            let version = dict
                                .get("auxinfo")
                                .and_then(|a| a.as_dictionary())
                                .and_then(|a| a.get("VERSION"))
                                .and_then(|v| v.as_string())
                                .unwrap_or("Unknown version");

                            let build = dict
                                .get("auxinfo")
                                .and_then(|a| a.as_dictionary())
                                .and_then(|a| a.get("BUILD"))
                                .and_then(|b| b.as_string())
                                .unwrap_or("Unknown build");

                            // Parse the version and compare with current version
                            if let Ok(update_version) = Version::parse(version) {
                                if update_version > current_version {
                                    Some(format!("macOS {} ({})", version, build))
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect(),
            other => {
                error!("Unexpected plist structure: {:?}", other);
                vec!["Unable to read updates: unexpected plist structure".to_string()]
            }
        };

        debug!("Found {} updates", updates.len());
        Ok(UpdatesInfo { updates })
    }
}

impl SegmentRenderer<UpdatesInfo> for UpdatesSegmentRenderer {
    fn height(&self) -> u16 {
        (self.info.updates.len()).max(1) as u16
    }

    fn render(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let [label_area, data_area] = create_label_data_layout(area);

        frame.render_widget(label("Updates"), label_area);

        let updates_text = if self.info.updates.is_empty() {
            "No updates available".to_string()
        } else {
            self.info.updates.join("\n")
        };

        frame.render_widget(Paragraph::new(updates_text), data_area);

        Ok(())
    }
}

impl From<Box<UpdatesInfo>> for UpdatesSegmentRenderer {
    fn from(info: Box<UpdatesInfo>) -> Self {
        Self { info: *info }
    }
}
