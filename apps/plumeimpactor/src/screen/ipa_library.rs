use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Alignment, Element, Fill, Length, Task};
use rust_i18n::t;
use std::path::PathBuf;
use plume_utils::Package;

use crate::appearance;

#[derive(Debug, Clone)]
pub struct IpaEntry {
    pub title: String,
    pub description: String,
    pub path: PathBuf,
}

#[derive(Debug, Clone)]
pub enum Message {
    InstallClicked(PathBuf),
    NavigateToInstaller(Package),
}

pub struct IpaLibraryScreen {
    pub entries: Vec<IpaEntry>,
}

const LIVE_CONTAINER_BYTES: &[u8] = include_bytes!("../../../../ipaDownloader/LiveContainer.ipa");

impl IpaLibraryScreen {
    pub fn new() -> Self {
        Self {
            entries: vec![IpaEntry {
                title: "LiveContainer".to_string(),
                description: t!("ipa_library_livecontainer_desc").to_string(),
                path: PathBuf::from("embedded://LiveContainer.ipa"),
            }],
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::InstallClicked(_) => {
                let temp_dir = std::env::temp_dir();
                let temp_path = temp_dir.join("LiveContainer_embedded.ipa");
                
                if let Err(e) = std::fs::write(&temp_path, LIVE_CONTAINER_BYTES) {
                    log::error!("Failed to write embedded IPA: {:?}", e);
                    return Task::none();
                }

                match Package::new(temp_path) {
                    Ok(package) => return Task::done(Message::NavigateToInstaller(package)),
                    Err(e) => {
                        log::error!("Failed to create package: {:?}", e);
                    }
                }
                Task::none()
            }
            Message::NavigateToInstaller(_) => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let mut content = column![].spacing(appearance::THEME_PADDING);

        for entry in &self.entries {
            let item_row = row![
                column![
                    text(&entry.title).size(appearance::THEME_FONT_SIZE + 4.0),
                    text(&entry.description)
                        .size(appearance::THEME_FONT_SIZE)
                        .style(|theme: &iced::Theme| iced::widget::text::Style {
                            color: Some(appearance::lighten(theme.palette().text, 0.7)),
                        })
                ]
                .spacing(5)
                .width(Length::FillPortion(3)),
                button(appearance::icon_text(appearance::DOWNLOAD, t!("install"), None))
                    .on_press(Message::InstallClicked(entry.path.clone()))
                    .style(appearance::p_button)
                    .width(Length::FillPortion(1))
            ]
            .spacing(appearance::THEME_PADDING)
            .align_y(Alignment::Center);

            content = content.push(
                container(item_row)
                    .padding(appearance::THEME_PADDING)
                    .style(|theme: &iced::Theme| container::Style {
                        background: Some(iced::Background::Color(appearance::lighten(
                            theme.palette().background,
                            0.05,
                        ))),
                        border: iced::Border {
                            width: 1.0,
                            color: appearance::lighten(theme.palette().background, 0.1),
                            radius: appearance::THEME_CORNER_RADIUS.into(),
                        },
                        ..Default::default()
                    }),
            );
        }

        container(scrollable(content)).height(Fill).into()
    }
}
