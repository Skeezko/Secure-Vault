mod crypto;
mod models;
mod storage;
use crate::models::{PasswordEntry, PasswordStore};
use crate::storage::Storage;
use iced::widget::button;
use iced::widget::pick_list;
use iced::widget::row;
use iced::widget::scrollable;
use iced::widget::text;
use iced::widget::text_input;
use iced::widget::{column, container};
use iced::{Alignment, clipboard};
use iced::{Element, Task, Theme};
use iced::{Fill, Font};

struct PasswordManagerApp {
    master_password_input: String,
    error_message: Option<String>,
    storage: Option<Storage>,
    store: PasswordStore,
    search_query: String,
    selected_service: Option<usize>,
    is_password_visible: bool,
    theme: Theme,
    new_service: String,
    new_user: String,
    new_password: String,
    password_length: u8,
}

#[derive(Debug, Clone)]
enum Message {
    MasterPasswordChanged(String),
    LoginPressed,
    SearchChanged(String),
    SelectService(usize),
    SaveEntry,
    DeleteService(usize),
    GenerateNewPassword(usize),
    ToggleVisibility,
    CopyPassword(String),
    ThemeChanged(Theme),
    NewServiceChanged(String),
    NewUserChanged(String),
    NewPasswordChanged(String),
    CancelEdit,
    LengthChanged(u8),
}

impl Default for PasswordManagerApp {
    fn default() -> Self {
        Self {
            master_password_input: String::new(),
            error_message: None,
            storage: None,
            store: PasswordStore {
                entries: Vec::new(),
            },
            search_query: String::new(),
            selected_service: None,
            is_password_visible: false,
            theme: Theme::Nightfly,
            new_service: String::new(),
            new_user: String::new(),
            new_password: String::new(),
            password_length: 16,
        }
    }
}

impl PasswordManagerApp {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ThemeChanged(theme) => {
                self.theme = theme;
            }
            Message::ToggleVisibility => {
                self.is_password_visible = !self.is_password_visible;
            }
            Message::MasterPasswordChanged(input) => {
                self.master_password_input = input;
            }
            Message::LoginPressed => match Storage::load(&self.master_password_input) {
                Ok((storage, store)) => {
                    self.storage = Some(storage);
                    self.store = store;
                    self.master_password_input.clear();
                    self.error_message = None;
                }
                Err(e) => self.error_message = Some(e),
            },
            Message::SelectService(id) => {
                self.selected_service = Some(id);
                let entry = &self.store.entries[id];
                self.new_service = entry.service.clone();
                self.new_user = entry.username.clone();
                self.new_password = entry.password.clone();
            }
            Message::SearchChanged(search) => {
                self.search_query = search;
            }
            Message::NewServiceChanged(s) => self.new_service = s,
            Message::NewUserChanged(u) => self.new_user = u,
            Message::NewPasswordChanged(p) => self.new_password = p,

            Message::GenerateNewPassword(len) => {
                if let Some(storage) = &self.storage {
                    self.new_password = storage.generate_password(len);
                }
            }
            Message::SaveEntry => {
                let new_entry = PasswordEntry {
                    service: self.new_service.clone(),
                    username: self.new_user.clone(),
                    password: self.new_password.clone(),
                };

                if let Some(id) = self.selected_service {
                    self.store.entries[id] = new_entry;
                } else {
                    self.store.entries.push(new_entry);
                }

                if let Some(storage) = &self.storage {
                    if let Err(e) = storage.save(self.store.clone()) {
                        self.error_message = Some(e);
                    }
                }

                self.new_service.clear();
                self.new_user.clear();
                self.new_password.clear();
                self.selected_service = None;
            }
            Message::DeleteService(id) => {
                self.store.entries.remove(id);
                if let Some(storage) = &self.storage {
                    let _ = storage.save(self.store.clone());
                }
                self.selected_service = None;
            }
            Message::CopyPassword(password) => {
                return clipboard::write(password);
            }
            Message::CancelEdit => {
                self.new_service.clear();
                self.new_user.clear();
                self.new_password.clear();
                self.selected_service = None;
            }
            Message::LengthChanged(new_len) => {
                self.password_length = new_len;
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        if self.storage.is_none() {
            self.view_login()
        } else {
            self.view_dashboard()
        }
    }

    fn view_login(&self) -> Element<'_, Message> {
        let title = text("SECURE VAULT").size(82).font(Font::MONOSPACE);

        let subtitle = text("Unlock your encrypted credentials").size(22);

        let theme_picker = row![
            text("Theme:").size(20),
            pick_list(Theme::ALL, Some(&self.theme), Message::ThemeChanged)
        ]
        .spacing(10)
        .align_y(Alignment::Center);

        let pwd_input = text_input("Enter Master Password", &self.master_password_input)
            .secure(!self.is_password_visible)
            .on_input(Message::MasterPasswordChanged)
            .on_submit(Message::LoginPressed)
            .padding(15)
            .size(18);

        let toggle_button = button(text(if self.is_password_visible {
            "󰈈"
        } else {
            "󰈉"
        }))
        .on_press(Message::ToggleVisibility)
        .padding(12)
        .style(button::secondary);

        let input_row = row![pwd_input, toggle_button]
            .spacing(10)
            .align_y(iced::Alignment::Center);

        let login_button = button(text("UNLOCK SYSTEM").font(Font::MONOSPACE))
            .on_press(Message::LoginPressed)
            .padding([12, 60])
            .style(button::primary);

        let error_text = if let Some(err) = &self.error_message {
            text(err).color([1.0, 0.3, 0.3]).size(14)
        } else {
            text("")
        };

        let login_page = container(
            column![
                title,
                subtitle,
                input_row,
                login_button,
                error_text,
                theme_picker
            ]
            .spacing(25)
            .align_x(iced::Alignment::Center),
        )
        .padding(40);

        container(login_page)
            .width(Fill)
            .height(Fill)
            .center(Fill)
            .into()
    }

    fn view_dashboard(&self) -> Element<'_, Message> {
        let sidebar = container(
            column![
                text("VAULT").size(24).font(Font::MONOSPACE),
                text_input("Search...", &self.search_query)
                    .on_input(Message::SearchChanged)
                    .padding(12),
                scrollable(
                    column(
                        self.store
                            .entries
                            .iter()
                            .enumerate()
                            .filter(|(_, e)| e
                                .service
                                .to_lowercase()
                                .contains(&self.search_query.to_lowercase()))
                            .map(|(i, e)| {
                                button(text(&e.service))
                                    .on_press(Message::SelectService(i))
                                    .width(Fill)
                                    .padding(10)
                                    .into()
                            })
                            .collect::<Vec<_>>()
                    )
                    .spacing(8)
                )
                .height(Fill)
            ]
            .spacing(20),
        )
        .width(280)
        .padding(20);

        let details_area = container(if let Some(id) = self.selected_service {
            let e = &self.store.entries[id];
            column![
                text(&e.service).size(50).font(Font::MONOSPACE),
                column![
                    text("Username").size(16),
                    text(&e.username).size(24).font(Font::MONOSPACE),
                ]
                .spacing(5)
                .align_x(Alignment::Center),
                column![
                    text("Password").size(16),
                    row![
                        text(if self.is_password_visible {
                            &e.password
                        } else {
                            "********"
                        })
                        .size(24)
                        .font(Font::MONOSPACE),
                        button(text(if self.is_password_visible {
                            "Hide"
                        } else {
                            "Show"
                        }))
                        .on_press(Message::ToggleVisibility)
                        .style(button::secondary),
                        button("Copy").on_press(Message::CopyPassword(e.password.clone())),
                    ]
                    .spacing(15)
                    .align_y(Alignment::Center),
                ]
                .spacing(5)
                .align_x(Alignment::Center),
                button("Delete Service")
                    .on_press(Message::DeleteService(id))
                    .style(button::danger)
                    .padding([10, 20]),
            ]
            .spacing(30)
            .align_x(Alignment::Center)
        } else {
            column![text("Select a service to view details").size(32)]
        })
        .width(Fill)
        .height(Fill)
        .center_x(Fill)
        .center_y(Fill);

        let add_form = container(
            column![
                text(if self.selected_service.is_some() {
                    "EDIT ENTRY"
                } else {
                    "NEW ENTRY"
                })
                .size(14),
                row![
                    text_input("Service", &self.new_service).on_input(Message::NewServiceChanged),
                    text_input("Username", &self.new_user).on_input(Message::NewUserChanged),
                ]
                .spacing(10),
                row![
                    text_input("Password", &self.new_password)
                        .on_input(Message::NewPasswordChanged),
                    row![
                        iced::widget::slider(8..=64, self.password_length, Message::LengthChanged)
                            .width(150),
                        text(format!("{}", self.password_length)).size(18).width(30),
                    ]
                    .spacing(10)
                    .align_y(Alignment::Center),
                    button("Gen")
                        .on_press(Message::GenerateNewPassword(self.password_length as usize)),
                ]
                .spacing(10)
                .align_y(Alignment::Center),
                row![
                    button(if self.selected_service.is_some() {
                        "Update"
                    } else {
                        "Save"
                    })
                    .on_press(Message::SaveEntry)
                    .width(Fill),
                    button("Cancel")
                        .on_press(Message::CancelEdit)
                        .style(button::secondary),
                ]
                .spacing(10)
            ]
            .spacing(15),
        )
        .padding(20);

        row![sidebar, column![details_area, add_form]].into()
    }
}

fn main() -> iced::Result {
    iced::application(
        PasswordManagerApp::default,
        PasswordManagerApp::update,
        PasswordManagerApp::view,
    )
    .title("Password Manager")
    .theme(|app: &PasswordManagerApp| app.theme.clone())
    .run()
}
