use iced::application::{StyleSheet, Appearance};
use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Element, Length, Sandbox, Settings, window, Theme, Color, Background};

pub fn run() -> iced::Result {
    let settings: Settings<()> = Settings {
        window: window::Settings {
            min_size: Some((520, 400)),
            max_size: Some((520, 400)),
            ..Default::default()
        },
        ..Default::default()
    };
    SpecifyGUI::run(settings)
}

#[derive(Default)]
struct SpecifyGUI {
    exit: bool
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Start,
    Exit
}

impl Sandbox for SpecifyGUI {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Specify")
    }

    fn should_exit(&self) -> bool {
        self.exit
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Start => {
                println!("lol started lmfao imagine");
                self.exit = true;
            }
            Message::Exit => {
                self.exit = true;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let heading = text("Specify").size(32);
        let description = text(
            "This tool gathers information about your computer. It does not gather any private information, other than perhaps your username."
        ).horizontal_alignment(iced::alignment::Horizontal::Center);
        let start = button(text("Start").horizontal_alignment(iced::alignment::Horizontal::Center)).on_press(Message::Start).width(iced::Length::Units(100)).style(iced::theme::Button::Primary);
        let exit = button(text("Exit").horizontal_alignment(iced::alignment::Horizontal::Center)).on_press(Message::Exit).width(iced::Length::Units(100)).style(iced::theme::Button::Secondary);

        let content = column![
            heading,
            description,
            row![
                start,
                exit
            ]
            .spacing(20)
            .padding([20, 0, 0, 0])
        ]
        .spacing(10)
        .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x()
            .center_y()
            .into()
    }

    fn style(&self) -> iced::theme::Application {
        iced::theme::Application::Custom( |_t| Appearance {
            background_color: Color::from_rgb8(0x2e, 0x34, 0x40),
            text_color: Color::from_rgb8(0xec, 0xef, 0xf4)
        })
    }
}
