use iced::{
    alignment, executor,
    widget::{button, column, radio, row, scrollable::Scrollable, text_input, Container, Text},
    window, Alignment, Application, Command, Length, Settings, Theme,
};
use std::{
    collections::HashMap,
    io::{self, Write},
    vec,
};

fn main() {
    let window_settings = window::Settings {
        size: (840, 600),
        min_size: Some((420, 300)),
        // max_size: todo!(),
        // resizable: false,
        // decorations: todo!(),
        // icon: todo!(),
        ..Default::default()
    };
    let uid = read().parse().unwrap();
    let username = read();
    let flag = Flag { username, uid };
    let settings = Settings {
        // id: todo!(),
        window: window_settings,
        flags: flag,
        // default_font: todo!(),
        ..Default::default()
    };
    App::run(settings).unwrap();
}

#[derive(Default)]
struct Flag {
    username: String,
    uid: i64,
}

#[derive(Default, Clone)]
struct App {
    username: String,
    uid: i64,
    friend_list: Vec<(String, i64)>,
    group_list: Vec<i64>,
    show_groups: bool,
    choose_chat_group: bool,
    join_id: String,
    showing_id: i64,
    dm_message: HashMap<i64, Vec<(String, String)>>,
    group_message: HashMap<i64, Vec<(String, String)>>,
    message: String,
}

#[derive(Debug, Clone)]
enum Message {
    JoinGroup,
    NewDm,
    RecievedMessage(String),
    ChangeShowGroups(bool),
    JoinIdChanged(String),
    SwitchChat((bool, i64)),
    ChangeInputMessage(String),
    Submit,
}

async fn recieve_message() -> String {
    read()
}

fn read() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let line = line.trim_end().to_string();
    eprintln!("UI: read message: {}", line);
    line
}

fn write(s: &str) {
    eprintln!("UI: Write message: {}", s);
    io::stdout().write_all(s.as_bytes()).unwrap();
    io::stdout().flush().unwrap();
}

impl Drop for App {
    fn drop(&mut self) {
        write("Exit\n");
    }
}

impl App {
    fn handle_message(&mut self, message: String) {
        let message = message.trim_end();
        match message {
            "add friend" => {
                let id = read().parse::<i64>().unwrap();
                let name = read();
                self.friend_list.push((name, id));
            }
            "DM" => {
                let n = read().parse::<i64>().unwrap() - 2; 
                let id = read().parse::<i64>().unwrap();
                let username = read();
                let mut message = String::new();
                for _ in 0..n {
                    message += read().as_str();
                    message.push('\n');
                }
                match self.dm_message.get_mut(&id) {
                    Some(x) => x.push((username, message)),
                    None => {
                        self.dm_message.insert(id, vec![(username, message)]);
                    }
                }
            }
            "group message" => {
                let n = read().parse::<i64>().unwrap() - 2;
                let gid = read().parse::<i64>().unwrap();
                let username = read();
                let mut message = String::new();
                for _ in 0..n {
                    message += read().as_str();
                    message.push('\n');
                }
                match self.group_message.get_mut(&gid) {
                    Some(x) => x.push((username, message)),
                    None => {
                        self.group_message.insert(gid, vec![(username, message)]);
                    }
                }
                // let vec = self.group_message.get_mut(&gid).unwrap();
                // vec.push((username, message));
            }
            _ => {
                eprintln!("unreachable message: {message}");
                // unreachable!()
            }
        }
    }
}

fn get_frined_list() -> Vec<(String, i64)> {
    let mut res = vec![];
    write("getFrinedList\n");
    loop {
        let id = read();
        let id: i64 = id.parse().unwrap();
        if id == -1 {
            break;
        }
        let name = read();
        res.push((name, id));
    }
    res
}

fn get_group_list() -> Vec<i64> {
    let mut res = vec![];
    write("getGroupList\n");
    loop {
        let id = read();
        let id: i64 = id.parse().unwrap();
        if id == -1 {
            break;
        }
        res.push(id);
    }
    res
}

impl Application for App {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = Flag;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let friend_list = get_frined_list();
        let group_list = get_group_list();
        write("START\n");
        let mut app: App = Default::default();
        app.username = flags.username;
        app.uid = flags.uid;
        app.friend_list = friend_list;
        app.group_list = group_list;
        (
            app,
            Command::perform(recieve_message(), |s| Message::RecievedMessage(s)),
        )
    }

    fn title(&self) -> String {
        format!("{} - {}", &self.username, self.uid)
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::JoinGroup => {
                if self.join_id.is_empty() {
                    return Command::none();
                }
                let id = self.join_id.parse().unwrap();
                if self.group_list.contains(&id) {
                    return Command::none();
                }
                write("joinGroup\n");
                write(self.join_id.as_str());
                write("\n");
                self.group_list.push(id);
                Command::none()
            }
            Message::NewDm => {
                if self.join_id.is_empty() {
                    return Command::none();
                }
                write("newDM\n");
                write(self.join_id.as_str());
                write("\n");
                Command::none()
            }
            Message::ChangeShowGroups(bool) => {
                self.show_groups = bool;
                Command::none()
            }
            Message::JoinIdChanged(s) => {
                if let Ok(_) = str::parse::<u64>(s.as_str()) {
                    self.join_id = s;
                } else if s.is_empty() {
                    self.join_id = s;
                }
                Command::none()
            }
            Message::RecievedMessage(msg) => {
                self.handle_message(msg);
                Command::perform(recieve_message(), |s| Message::RecievedMessage(s))
            }
            Message::SwitchChat((group, id)) => {
                self.choose_chat_group = group;
                self.showing_id = id;
                Command::none()
            }
            Message::Submit => {
                if self.message.is_empty() {
                    return Command::none();
                }
                if self.choose_chat_group {
                    write(&format!(
                        "GROUP\n{}\n{}\n{}\n",
                        self.uid, self.showing_id, self.message
                    ));
                } else {
                    write(&format!(
                        "DM\n{}\n{}\n{}\n",
                        self.uid, self.showing_id, self.message
                    ));
                    match self.dm_message.get_mut(&self.showing_id) {
                        Some(x) => {
                            x.push((self.username.clone(), self.message.clone()));
                        }
                        None => {
                            self.dm_message.insert(
                                self.showing_id,
                                vec![(self.username.clone(), self.message.clone())],
                            );
                        }
                    }
                }
                self.message.clear();
                Command::none()
            }
            Message::ChangeInputMessage(s) => {
                self.message = s;
                Command::none()
            } // _ => Command::none(),
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let switch_list = row![
            button(
                Text::new("DMs")
                    .vertical_alignment(alignment::Vertical::Center)
                    .horizontal_alignment(alignment::Horizontal::Center)
            )
            .width(105)
            // .height(Length::Fill)
            .on_press(Message::ChangeShowGroups(false)),
            button(
                Text::new("Groups")
                    .vertical_alignment(alignment::Vertical::Center)
                    .horizontal_alignment(alignment::Horizontal::Center)
            )
            .width(105)
            // .height(Length::Fill)
            .on_press(Message::ChangeShowGroups(true)),
        ]
        .align_items(Alignment::Center)
        .height(30)
        .width(Length::Fill);

        let mut list = vec![];
        if self.show_groups {
            for id in self.group_list.iter() {
                list.push(
                    radio(
                        format!("{id}"),
                        (true, *id),
                        Some((self.choose_chat_group, self.showing_id)),
                        Message::SwitchChat,
                    )
                    .into(),
                );
            }
        } else {
            for (name, id) in self.friend_list.iter() {
                list.push(
                    radio(
                        name,
                        (false, *id),
                        Some((self.choose_chat_group, self.showing_id)),
                        Message::SwitchChat,
                    )
                    .into(),
                );
            }
        }

        let list = Container::new(
            Scrollable::new(Container::new(column(list)).width(Length::Fill)), // .vertical_scroll(Properties::new()),
        )
        .height(Length::Fill)
        .width(Length::Fill);

        let join_area = column![
            text_input("input group id or user id", &self.join_id)
                .on_input(|s| Message::JoinIdChanged(s)),
            row![
                button(
                    Text::new("new DM")
                        .vertical_alignment(alignment::Vertical::Center)
                        .horizontal_alignment(alignment::Horizontal::Center)
                )
                .width(105)
                // .height(Length::Fill)
                .on_press(Message::NewDm),
                button(
                    Text::new("Join Group")
                        .vertical_alignment(alignment::Vertical::Center)
                        .horizontal_alignment(alignment::Horizontal::Center)
                )
                .width(105)
                // .height(Length::Fill)
                .on_press(Message::JoinGroup),
            ]
            .align_items(Alignment::Center)
            .height(30)
            .width(Length::Fill),
        ]
        .height(60);

        let side_bar = column![switch_list, list, join_area].width(210);

        let mut message_record = vec![];

        if self.choose_chat_group {
            if self.group_message.contains_key(&self.showing_id) {
                for (name, message) in self.group_message[&self.showing_id].iter() {
                    message_record.push(Text::new(name).size(10).into());
                    message_record.push(Text::new(message).into());
                }
            }
        } else {
            if self.dm_message.contains_key(&self.showing_id) {
                for (name, message) in self.dm_message[&self.showing_id].iter() {
                    message_record.push(Text::new(name).size(10).into());
                    message_record.push(Text::new(message).into());
                }
            }
        }
        let chat_area = column![
            Container::new(Scrollable::new(
                Container::new(column(message_record)).width(Length::Fill),
            ))
            .width(Length::Fill)
            .height(Length::Fill),
            text_input("", &self.message)
                .width(Length::Fill)
                .on_input(Message::ChangeInputMessage)
                .on_submit(Message::Submit),
        ]
        .width(Length::Fill)
        .height(Length::Fill);

        let content = row![side_bar, chat_area].into();

        content // .explain(Color::BLACK)
    }
}
