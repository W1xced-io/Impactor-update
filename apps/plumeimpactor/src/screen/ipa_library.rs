use iced::widget::{button, column, container, row, scrollable, text, progress_bar, text_input};
use iced::{Alignment, Element, Fill, Length, Task};
use rust_i18n::t;
use std::path::PathBuf;
use plume_utils::Package;
use iced::futures::channel::mpsc;
use std::io::Write;
use serde::{Deserialize, Serialize};

use crate::appearance;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpaEntry {
    pub title: String,
    pub description: String,
    pub download_url: String,
    #[serde(default)]
    pub icon_url: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    FetchLibrary,
    LibraryFetched(Vec<IpaEntry>),
    LibraryFetchError(String),
    SearchChanged(String),
    DownloadClicked(IpaEntry),
    DownloadProgress(f32),
    DownloadFinished(PathBuf),
    DownloadError(String),
    NavigateToInstaller(Package),
}

pub struct IpaLibraryScreen {
    pub entries: Vec<IpaEntry>,
    pub filtered_entries: Vec<IpaEntry>,
    pub search_query: String,
    pub downloading_entry: Option<String>,
    pub download_progress: f32,
    pub is_loading: bool,
    pub error: Option<String>,
}

const LIBRARY_JSON_URL: &str = "https://raw.githubusercontent.com/W1xced-io/Impactor-update/main/apps.json";

impl IpaLibraryScreen {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            filtered_entries: Vec::new(),
            search_query: String::new(),
            downloading_entry: None,
            download_progress: 0.0,
            is_loading: true,
            error: None,
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::FetchLibrary => {
                self.is_loading = true;
                self.error = None;
                let (tx, rx) = mpsc::unbounded();

                std::thread::spawn(move || {
                    match fetch_library_blocking() {
                        Ok(entries) => {
                            let _ = tx.unbounded_send(Message::LibraryFetched(entries));
                        }
                        Err(e) => {
                            let _ = tx.unbounded_send(Message::LibraryFetchError(e));
                        }
                    }
                });

                return Task::stream(rx);
            }
            Message::LibraryFetched(entries) => {
                self.is_loading = false;
                self.entries = entries.clone();
                self.filtered_entries = entries;
                Task::none()
            }
            Message::LibraryFetchError(e) => {
                self.is_loading = false;
                self.error = Some(e);
                Task::none()
            }
            Message::SearchChanged(query) => {
                self.search_query = query.clone();
                let lower_query = query.to_lowercase();
                self.filtered_entries = self.entries.iter()
                    .filter(|e| e.title.to_lowercase().contains(&lower_query) || e.description.to_lowercase().contains(&lower_query))
                    .cloned()
                    .collect();
                Task::none()
            }
            Message::DownloadClicked(entry) => {
                self.downloading_entry = Some(entry.title.clone());
                self.download_progress = 0.0;
                let url = entry.download_url.clone();
                let title = entry.title.clone();

                let (mut tx, rx) = mpsc::unbounded();

                std::thread::spawn(move || {
                    if let Err(e) = download_blocking(url, title, &mut tx) {
                        let _ = tx.unbounded_send(Message::DownloadError(e));
                    }
                });

                return Task::stream(rx);
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
        if self.is_loading {
            return container(
                column![
                    text(t!("loading")).size(20),
                    progress_bar(0.0..=100.0, 0.0).style(appearance::p_progress_bar)
                ]
                .spacing(10)
                .align_x(Alignment::Center)
            )
            .center_x(Fill)
            .center_y(Fill)
            .width(Fill)
            .height(Fill)
            .into();
        }

        if let Some(ref err) = self.error {
            return container(
                column![
                    text(format!("Error: {}", err)).style(|_theme: &iced::Theme| iced::widget::text::Style { color: Some(iced::color!(0xff0000)) }),
                    button(text("Retry")).on_press(Message::FetchLibrary).style(appearance::p_button)
                ]
                .spacing(10)
                .align_x(Alignment::Center)
            )
            .center_x(Fill)
            .center_y(Fill)
            .width(Fill)
            .height(Fill)
            .into();
        }

        let search_input = text_input(&t!("search").to_string(), &self.search_query)
            .on_input(Message::SearchChanged)
            .padding(appearance::THEME_PADDING)
            .size(appearance::THEME_FONT_SIZE + 2.0);

        let mut content = column![search_input].spacing(appearance::THEME_PADDING);

        if self.filtered_entries.is_empty() {
             content = content.push(
                container(text(t!("nothing_found").to_string()).size(16))
                    .width(Fill)
                    .padding(50)
                    .center_x(Fill)
             );
        }

        for entry in &self.filtered_entries {
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

fn fetch_library_blocking() -> Result<Vec<IpaEntry>, String> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("PlumeImpactor")
        .build()
        .map_err(|e| e.to_string())?;

    let response = client.get(LIBRARY_JSON_URL)
        .send()
        .map_err(|e| e.to_string())?;

    let entries: Vec<IpaEntry> = response.json().map_err(|e| e.to_string())?;
    Ok(entries)
}

fn download_blocking(url: String, title: String, tx: &mut mpsc::UnboundedSender<Message>) -> Result<(), String> {
    log::info!("Starting blocking download for {} from {}", title, url);
    let client = reqwest::blocking::Client::builder()
        .user_agent("PlumeImpactor")
        .build()
        .map_err(|e| e.to_string())?;

    let mut response = client.get(url).send().map_err(|e| e.to_string())?;

    let total_size = response.content_length();
    let temp_dir = std::env::temp_dir();
    let file_name = format!("{}.ipa", title.replace(' ', "_"));
    let dest_path = temp_dir.join(file_name);

    let mut file = std::fs::File::create(&dest_path).map_err(|e| e.to_string())?;

    let mut downloaded: u64 = 0;
    let mut buffer = [0u8; 8192];

    loop {
        let n = response.read(&mut buffer).map_err(|e| e.to_string())?;
        if n == 0 {
            break;
        }

        file.write_all(&buffer[..n]).map_err(|e| e.to_string())?;
        downloaded += n as u64;

        if let Some(total) = total_size {
            let progress = downloaded as f32 / total as f32;
            let _ = tx.unbounded_send(Message::DownloadProgress(progress));
        }
    }

    let _ = tx.unbounded_send(Message::DownloadFinished(dest_path));
    Ok(())
}

use std::io::Read;