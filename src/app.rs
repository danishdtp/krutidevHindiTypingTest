pub static EMBEDDED_WORDS: &str = include_str!("../hindi-words.txt");

use iced::widget::{button, container, text_input, Column, Row, Text};
use iced::{Element, Font, Length, Subscription};
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use std::time::Instant;

use crate::db::Database;
use crate::models::Duration as AppDuration;

pub struct App {
    words: Vec<String>,
    target_words: Vec<String>,
    typed_text: String,
    duration: AppDuration,
    remaining_time: u64,
    start_time: Option<Instant>,
    running: bool,
    completed: bool,
    wpm: i32,
    accuracy: i32,
    best_wpm: i32,
    current_word_index: usize,
    current_page: usize,
    error_positions: Vec<usize>,
    db: Database,
}

impl Default for App {
    fn default() -> Self {
        let words: Vec<String> = EMBEDDED_WORDS
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        let db = Database::new("typing_stats.db").unwrap();
        let best_wpm = db.get_best_wpm().unwrap_or(0);

        let duration = AppDuration::Seconds15;
        let target_words = Self::generate_words(&words, duration.word_count() as usize);

        App {
            words,
            target_words,
            typed_text: String::new(),
            duration,
            remaining_time: duration.as_secs(),
            start_time: None,
            running: false,
            completed: false,
            wpm: 0,
            accuracy: 100,
            best_wpm,
            current_word_index: 0,
            current_page: 0,
            error_positions: Vec::new(),
            db,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    DurationSelected(AppDuration),
    LoadFile,
    Reset,
    StopTest,
    NextPage,
    PrevPage,
    Tick,
}

impl App {
    pub fn new(words: Vec<String>, db: Database) -> Self {
        let duration = AppDuration::Seconds15;
        let target_words = Self::generate_words(&words, duration.word_count() as usize);
        let best_wpm = db.get_best_wpm().unwrap_or(0);

        App {
            words,
            target_words,
            typed_text: String::new(),
            duration,
            remaining_time: duration.as_secs(),
            start_time: None,
            running: false,
            completed: false,
            wpm: 0,
            accuracy: 100,
            best_wpm,
            current_word_index: 0,
            current_page: 0,
            error_positions: Vec::new(),
            db,
        }
    }

    fn generate_words(all_words: &[String], count: usize) -> Vec<String> {
        let mut rng = StdRng::from_entropy();
        all_words
            .choose_multiple(&mut rng, count)
            .cloned()
            .collect()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::InputChanged(text) => {
                if self.completed {
                    return;
                }

                if text.len() < self.typed_text.len() {
                    self.typed_text = text;
                    self.update_stats();
                    return;
                }

                self.typed_text = text;

                if !self.running && !self.typed_text.is_empty() {
                    self.running = true;
                    self.start_time = Some(Instant::now());
                }

                self.update_stats();

                if self.running && !self.completed {
                    if let Some(start) = self.start_time {
                        let elapsed = start.elapsed().as_secs();
                        if elapsed >= self.duration.as_secs() {
                            self.finish_test();
                            return;
                        }
                        self.remaining_time = self.duration.as_secs() - elapsed;
                    }
                }

                if self.typed_text == self.target_words.join(" ") {
                    self.finish_test();
                }
            }

            Message::DurationSelected(duration) => {
                if self.running {
                    return;
                }
                self.duration = duration;
                self.remaining_time = duration.as_secs();
                self.generate_new_text();
            }

            Message::LoadFile => {
                if let Some(path) = native_dialog::FileDialog::new()
                    .add_filter("Text files", &["txt"])
                    .show_open_single_file()
                    .ok()
                    .flatten()
                {
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        let words: Vec<String> =
                            content.split_whitespace().map(|s| s.to_string()).collect();
                        if !words.is_empty() {
                            self.words = words;
                            self.generate_new_text();
                        }
                    }
                }
            }

            Message::Reset => {
                self.running = false;
                self.completed = false;
                self.start_time = None;
                self.remaining_time = self.duration.as_secs();
                self.typed_text = String::new();
                self.wpm = 0;
                self.accuracy = 100;
                self.current_word_index = 0;
                self.current_page = 0;
                self.error_positions = Vec::new();
                self.generate_new_text();
            }

            Message::StopTest => {
                if self.running || !self.typed_text.is_empty() {
                    self.finish_test();
                }
            }

            Message::NextPage => {
                let total_pages =
                    (self.target_words.len() + self.words_per_page() - 1) / self.words_per_page();
                if self.current_page + 1 < total_pages {
                    self.current_page += 1;
                }
            }

            Message::PrevPage => {
                if self.current_page > 0 {
                    self.current_page -= 1;
                }
            }

            Message::Tick => {
                if self.running && !self.completed {
                    if let Some(start) = self.start_time {
                        let elapsed = start.elapsed().as_secs();
                        if elapsed >= self.duration.as_secs() {
                            self.finish_test();
                        } else {
                            self.remaining_time = self.duration.as_secs() - elapsed;
                            self.update_stats();
                        }
                    }
                }
            }
        }
    }

    fn generate_new_text(&mut self) {
        self.target_words = Self::generate_words(&self.words, self.duration.word_count() as usize);
        self.typed_text = String::new();
        self.current_word_index = 0;
        self.current_page = 0;
        self.error_positions = Vec::new();
    }

    fn words_per_page(&self) -> usize {
        self.target_words.len()
    }

    fn update_stats(&mut self) {
        let typed_words: Vec<&str> = self.typed_text.split_whitespace().collect();
        let target_words: Vec<&str> = self.target_words.iter().map(|s| s.as_str()).collect();

        self.current_word_index = typed_words.len().saturating_sub(1);

        let words_per_page = self.words_per_page();
        let total_pages =
            ((self.target_words.len() as f64) / (words_per_page as f64)).ceil() as usize;

        if total_pages > 1 {
            let new_page = self.current_word_index / words_per_page;
            if new_page != self.current_page && new_page < total_pages {
                self.current_page = new_page;
            }
        }

        if !typed_words.is_empty() {
            let mut correct_words = 0;
            for (i, typed_word) in typed_words.iter().enumerate() {
                if i < target_words.len() {
                    if *typed_word == target_words[i] {
                        correct_words += 1;
                    } else if !self.error_positions.contains(&i) {
                        self.error_positions.push(i);
                    }
                }
            }

            let total_chars: usize = self
                .typed_text
                .chars()
                .filter(|c| !c.is_whitespace())
                .count();
            if total_chars > 0 {
                let elapsed = self
                    .start_time
                    .map(|t| t.elapsed().as_secs_f64())
                    .unwrap_or(1.0);
                self.wpm = ((self.typed_text.len() as f64 / 5.0) / (elapsed / 60.0)) as i32;
                self.wpm = self.wpm.max(0);

                let typed_word_count = typed_words.len() as i32;
                if typed_word_count > 0 {
                    self.accuracy =
                        ((correct_words as f64 / typed_word_count as f64) * 100.0) as i32;
                    self.accuracy = self.accuracy.max(0).min(100);
                }
            }
        } else {
            self.wpm = 0;
            self.accuracy = 100;
        }
    }

    fn finish_test(&mut self) {
        self.completed = true;
        self.running = false;

        let date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let _ = self.db.insert_score(self.wpm, self.accuracy, &date);

        if let Ok(best) = self.db.get_best_wpm() {
            self.best_wpm = best;
        }

        let wpm = self.wpm;
        let acc = self.accuracy;

        std::thread::spawn(move || {
            let _ = native_dialog::MessageDialog::new()
                .set_title("Test Complete")
                .set_text(&format!("Speed: {} WPM\nAccuracy: {}%", wpm, acc))
                .set_type(native_dialog::MessageType::Info)
                .show_alert();
        });
    }

    pub fn view(&self) -> Element<Message> {
        let header = container(
            Row::new()
                .spacing(30)
                .push(
                    Text::new(format!("WPM: {}", self.wpm))
                        .size(24)
                        .color(iced::Color::WHITE),
                )
                .push(
                    Text::new(format!("Accuracy: {}%", self.accuracy))
                        .size(24)
                        .color(iced::Color::WHITE),
                )
                .push(
                    Text::new(format!("Best: {} WPM", self.best_wpm))
                        .size(20)
                        .color(iced::Color::from_rgb(0.95, 0.77, 0.06)),
                )
                .push(
                    Text::new("Hindi Typing Practice - By Danish Khan")
                        .size(12)
                        .color(iced::Color::from_rgb(0.7, 0.7, 0.7)),
                ),
        )
        .width(Length::Fill)
        .padding(20);

        let header_bg = container(header)
            .width(Length::Fill)
            .height(Length::Fixed(80.0))
            .style(|_theme| -> container::Style {
                container::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgb(
                        0.17, 0.24, 0.31,
                    ))),
                    ..Default::default()
                }
            });

        let durations = [
            (AppDuration::Seconds15, "15s"),
            (AppDuration::Seconds30, "30s"),
            (AppDuration::Minute1, "1m"),
        ];

        let duration_buttons: Element<Message> = Row::new()
            .spacing(10)
            .push(Text::new("Duration:").size(16))
            .extend(durations.iter().map(|(dur, label)| {
                let is_selected = self.duration == *dur;
                button(Text::new(*label).size(16))
                    .width(Length::Fixed(70.0))
                    .height(Length::Fixed(40.0))
                    .on_press(Message::DurationSelected(*dur))
                    .style(move |_state, _theme| -> button::Style {
                        if is_selected {
                            button::Style {
                                background: Some(iced::Background::Color(iced::Color::from_rgb(
                                    0.2, 0.6, 0.86,
                                ))),
                                text_color: iced::Color::WHITE,
                                ..Default::default()
                            }
                        } else {
                            button::Style {
                                background: Some(iced::Background::Color(iced::Color::from_rgb(
                                    0.9, 0.9, 0.9,
                                ))),
                                text_color: iced::Color::BLACK,
                                ..Default::default()
                            }
                        }
                    })
                    .into()
            }))
            .push(Text::new(format!("Time: {}s", self.remaining_time)).size(16))
            .into();

        let duration_row = container(duration_buttons).width(Length::Fill).padding(20);

        let words_per_page = self.words_per_page();
        let total_pages =
            ((self.target_words.len() as f64) / (words_per_page as f64)).ceil() as usize;

        let page_info = if total_pages > 1 {
            Text::new(format!("Page {} of {}", self.current_page + 1, total_pages)).size(14)
        } else {
            Text::new("").size(14)
        };

        let prev_button = if total_pages > 1 && self.current_page > 0 {
            button(Text::new("← Prev").size(14))
                .on_press(Message::PrevPage)
                .padding(5)
        } else {
            button(Text::new("").size(14))
        };

        let next_button = if total_pages > 1 && self.current_page + 1 < total_pages {
            button(Text::new("Next →").size(14))
                .on_press(Message::NextPage)
                .padding(5)
        } else {
            button(Text::new("").size(14))
        };

        let page_nav = Row::new()
            .spacing(10)
            .push(prev_button)
            .push(page_info)
            .push(next_button);

        let page_row = container(page_nav).width(Length::Fill).padding(5);

        let num_lines = match self.duration {
            AppDuration::Seconds15 => 3,
            AppDuration::Seconds30 => 5,
            AppDuration::Minute1 => 6,
        };
        let word_size: f32 = if num_lines >= 6 { 28.0 } else { 32.0 };

        let start_idx = self.current_page * words_per_page;
        let end_idx = (start_idx + words_per_page).min(self.target_words.len());
        let page_words = &self.target_words[start_idx..end_idx];

        let words_per_line = match self.duration {
            AppDuration::Seconds15 => 5,
            AppDuration::Seconds30 => 6,
            AppDuration::Minute1 => 10,
        };

        let typed_words: Vec<&str> = self.typed_text.split_whitespace().collect();
        let num_lines = (page_words.len() + words_per_line - 1) / words_per_line;

        let lines: Vec<Element<Message>> = (0..num_lines)
            .map(|line_idx| {
                let start_word = line_idx * words_per_line;
                let end_word = (start_word + words_per_line).min(page_words.len());
                let line_words: Vec<Element<Message>> = page_words[start_word..end_word]
                    .iter()
                    .enumerate()
                    .map(|(i, word)| {
                        let idx = start_idx + start_word + i;
                        let color = if idx < typed_words.len() {
                            if typed_words[idx] == word.as_str() {
                                iced::Color::from_rgb(0.2, 0.7, 0.3)
                            } else {
                                iced::Color::from_rgb(0.9, 0.2, 0.2)
                            }
                        } else if idx == typed_words.len() {
                            iced::Color::from_rgb(0.2, 0.4, 0.8)
                        } else {
                            iced::Color::from_rgb(0.3, 0.3, 0.3)
                        };

                        Text::new(word.as_str())
                            .size(word_size)
                            .font(Font::with_name("Kruti Dev 010"))
                            .color(color)
                            .into()
                    })
                    .collect();
                Row::new().spacing(8).extend(line_words).into()
            })
            .collect();

        let target_text = Column::new().spacing(8).extend(lines);

        let text_area = container(target_text)
            .width(Length::Fill)
            .padding(20)
            .style(|_theme| -> container::Style {
                container::Style {
                    background: Some(iced::Background::Color(iced::Color::WHITE)),
                    ..Default::default()
                }
            });

        let input = text_input(";gka Vkbi djsa ...", &self.typed_text)
            .on_input(Message::InputChanged)
            .size(28)
            .font(Font::with_name("Kruti Dev 010"))
            .padding(15)
            .width(Length::Fill);

        let input_container = container(input).width(Length::Fill).padding(20);

        let load_button = button(Text::new("Load File").size(16))
            .width(Length::Fixed(140.0))
            .height(Length::Fixed(45.0))
            .on_press(Message::LoadFile)
            .style(|_state, _theme| -> button::Style {
                button::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgb(
                        0.2, 0.6, 0.86,
                    ))),
                    text_color: iced::Color::WHITE,
                    ..Default::default()
                }
            });

        let stop_button = button(Text::new("Stop Test").size(16))
            .width(Length::Fixed(140.0))
            .height(Length::Fixed(45.0))
            .on_press(Message::StopTest)
            .style(|_state, _theme| -> button::Style {
                button::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgb(
                        0.8, 0.5, 0.2,
                    ))),
                    text_color: iced::Color::WHITE,
                    ..Default::default()
                }
            });

        let reset_button = button(Text::new("Reset Test").size(16))
            .width(Length::Fixed(140.0))
            .height(Length::Fixed(45.0))
            .on_press(Message::Reset)
            .style(|_state, _theme| -> button::Style {
                button::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgb(
                        0.9, 0.3, 0.3,
                    ))),
                    text_color: iced::Color::WHITE,
                    ..Default::default()
                }
            });

        let button_row = Row::new()
            .spacing(20)
            .push(load_button)
            .push(stop_button)
            .push(reset_button);

        let button_container = container(button_row).width(Length::Fill).padding(20);

        Column::new()
            .spacing(0)
            .push(header_bg)
            .push(duration_row)
            .push(page_row)
            .push(text_area)
            .push(input_container)
            .push(button_container)
            .into()
    }
}
