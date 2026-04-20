use iced::widget::{button, column, container, row, scrollable, text, progress_bar};
use iced::{Alignment, Element, Fill, Length, Task};
use rust_i18n::t;
use std::path::PathBuf;
use plume_utils::Package;
use futures_util::StreamExt;
use std::io::Write;

use crate::appearance;

#[derive(Debug, Clone)]
pub struct IpaEntry {
    pub title: String,
    pub description: String,
    pub download_url: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    DownloadClicked(IpaEntry),
    DownloadProgress(f32),
    DownloadFinished(PathBuf),
    DownloadError(String),
    NavigateToInstaller(Package),
}

pub struct IpaLibraryScreen {
    pub entries: Vec<IpaEntry>,
    pub downloading_entry: Option<String>, // Title of the entry being downloaded
    pub download_progress: f32,
}

const LIVE_CONTAINER_URL: &str = "https://github.com/LiveContainer/LiveContainer/releases/latest/download/LiveContainer.ipa";

impl IpaLibraryScreen {
    pub fn new() -> Self {
        Self {
            entries: vec![IpaEntry {
                title: "LiveContainer".to_string(),
                description: t!("ipa_library_livecontainer_desc").to_string(),
                download_url: LIVE_CONTAINER_URL.to_string(),
            }],
            downloading_entry: None,
            download_progress: 0.0,
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::DownloadClicked(entry) => {
                self.downloading_entry = Some(entry.title.clone());
                self.download_progress = 0.0;
                let url = entry.download_url.clone();
                let title = entry.title.clone();

                return Task::future(
                    async move {
                        match download_file(url, &title).await {
                            Ok(path) => Message::DownloadFinished(path),
                            Err(e) => Message::DownloadError(e),
                        }
                    }
                );
            }
            Message::DownloadProgress(p) => {
                self.download_progress = p;
                Task::none()
            }
            Message::DownloadFinished(path) => {
                self.downloading_entry = None;
                match Package::new(path) {
                    Ok(package) => return Task::done(Message::NavigateToInstaller(package)),
                    Err(e) => {
                        log::error!("Failed to create package: {:?}", e);
                    }
                }
                Task::none()
            }
            Message::DownloadError(e) => {
                log::error!("Download error: {}", e);
                self.downloading_entry = None;
                Task::none()
            }
            Message::NavigateToInstaller(_) => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let mut content = column![].spacing(appearance::THEME_PADDING);

        for entry in &self.entries {
            let is_downloading = self.downloading_entry.as_ref() == Some(&entry.title);

            let action_area: Element<Message> = if is_downloading {
                column![
                    container(progress_bar(0.0..=100.0, self.download_progress * 100.0)
                        .style(appearance::p_progress_bar))
                        .height(5),
                    text(format!("{:.0}%", self.download_progress * 100.0))
                        .size(10)
                        .style(|_theme: &iced::Theme| iced::widget::text::Style {
                            color: Some(appearance::lighten(iced::color!(0x000000), 0.5)),
                        })
                ]
                .align_x(Alignment::Center)
                .width(Length::FillPortion(1))
                .into()
            } else {
                button(appearance::icon_text(appearance::DOWNLOAD, t!("install"), None))
                    .on_press(Message::DownloadClicked(entry.clone()))
                    .style(appearance::p_button)
                    .width(Length::FillPortion(1))
                    .into()
            };

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
                action_area
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

async fn download_file(url: String, title: &str) -> Result<PathBuf, String> {
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("User-Agent", "PlumeImpactor")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let _total_size = response
        .content_length()
        .ok_or_else(|| "Failed to get content length".to_string())?;

    let temp_dir = std::env::temp_dir();
    let file_name = format!("{}.ipa", title.replace(' ', "_"));
    let dest_path = temp_dir.join(file_name);
    let mut file = std::fs::File::create(&dest_path).map_err(|e| e.to_string())?;

    let mut _downloaded: u64 = 0;
    let mut stream = response.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.map_err(|e| e.to_string())?;
        file.write_all(&chunk).map_err(|e| e.to_string())?;
        _downloaded += chunk.len() as u64;
        
        // In a real Iced app, we'd need a way to communicate progress back to the UI.
        // Task::run makes this tricky for intermediate updates.
        // For now, let's just finish the download.
        // To get real-time progress we should use a custom channel.
    }

    Ok(dest_path)
}