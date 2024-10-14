#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use iced::{
    event,
    keyboard::key::Named,
    widget::{column, Column},
    window, Event, Subscription, Task,
};

pub fn main() -> iced::Result {
    iced::application("A cool counter", App::update, App::view)
        .theme(App::theme)
        .subscription(App::subscription)
        .run_with(App::new)
}

#[derive(Default)]
struct App {}

#[derive(Debug, Clone)]
enum Message {
    EventOccurred(Event),
}

impl App {
    fn new() -> (Self, Task<Message>) {
        (
            Self::default(),
            window::get_latest()
                .and_then(|window| window::change_mode(window, window::Mode::Fullscreen)),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::EventOccurred(event) => {
                if let Event::Keyboard(e) = event {
                    if let iced::keyboard::Event::KeyPressed { key, .. } = e {
                        if let iced::keyboard::Key::Named(Named::Escape) = key {
                            return window::get_latest().and_then(window::close);
                        }
                    }
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Column<Message> {
        column![]
    }

    fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::EventOccurred)
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Light
    }
}
