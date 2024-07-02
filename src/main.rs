use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::Alignment,
    prelude::*,
    symbols::border,
    widgets::{
        block::{Position, *},
        *,
    },
};

mod tui;

#[derive(Debug, Clone)]
struct Task {
    content: Vec<char>,
}

#[derive(Debug, PartialEq, Eq)]
enum AppState {
    Normal,
    CreateTask,
}

impl Default for AppState {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Debug, Default)]
pub struct App {
    tasks: Vec<Vec<Task>>,
    state: AppState,
    exit: bool,
    buffer: Vec<char>,
    cursor_pos: (usize, usize),
}

impl App {
    fn default() -> Self {
        App {
            tasks: vec![Vec::new(), Vec::new(), Vec::new()],
            state: AppState::default(),
            exit: false,
            buffer: Vec::new(),
            cursor_pos: (0, 0),
        }
    }

    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match self.state {
            AppState::Normal => match key_event.code {
                KeyCode::Char('q') => self.exit(),
                KeyCode::Char('n') => self.create_task(),
                KeyCode::Char('d') => self.update_task_state(None),
                KeyCode::Left => self.move_left(),
                KeyCode::Right => self.move_right(),
                KeyCode::Up => self.move_up(),
                KeyCode::Down => self.move_down(),
                KeyCode::Char('1') => self.update_task_state(Some(0)),
                KeyCode::Char('2') => self.update_task_state(Some(1)),
                KeyCode::Char('3') => self.update_task_state(Some(2)),
                _ => {}
            },
            AppState::CreateTask => match key_event.code {
                KeyCode::Char(input) => self.write_char_to_buffer(input),
                KeyCode::Backspace => self.delete_char_from_buffer(),
                KeyCode::Enter => self.submit_task(),
                KeyCode::Esc => self.cancel_popup(),
                _ => {}
            },
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn create_task(&mut self) {
        self.state = AppState::CreateTask;
    }

    fn write_char_to_buffer(&mut self, input: char) {
        self.buffer.push(input);
    }

    fn delete_char_from_buffer(&mut self) {
        self.buffer.pop();
    }

    fn cancel_popup(&mut self) {
        self.buffer = Vec::new();
        self.state = AppState::Normal;
    }

    fn move_right(&mut self) {
        let new_pos = self.cursor_pos.0 + 1;
        if self.cursor_pos.0 < 2 {
            self.cursor_pos.0 = new_pos;
            self.correct_cursor(new_pos);
        }
    }

    fn move_left(&mut self) {
        if self.cursor_pos.0 > 0 {
            let new_pos = self.cursor_pos.0 - 1;
            self.cursor_pos.0 = new_pos;
            self.correct_cursor(new_pos);
        }
    }

    fn correct_cursor(&mut self, new_pos: usize) {
        if self.tasks[new_pos].is_empty() {
            self.cursor_pos.1 = 0;
        } else if self.tasks[new_pos].len() - 1 < self.cursor_pos.1 {
            self.cursor_pos.1 = self.tasks[new_pos].len() - 1;
        }
    }

    fn move_up(&mut self) {
        if self.cursor_pos.1 > 0 {
            self.cursor_pos.1 -= 1;
        }
    }

    fn move_down(&mut self) {
        if !self.tasks[self.cursor_pos.0].is_empty()
            && self.cursor_pos.1 < self.tasks[self.cursor_pos.0].len() - 1
        {
            self.cursor_pos.1 += 1;
        }
    }

    fn update_task_state(&mut self, new_state: Option<usize>) {
        if new_state.is_some() && self.cursor_pos.0 == new_state.unwrap() {
            return;
        }
        match self.tasks[self.cursor_pos.0].len() {
            0 => {
                return;
            }
            1 => {
                self.cursor_pos.1 = 0;
            }
            _ => {
                if self.cursor_pos.1 == self.tasks[self.cursor_pos.0].len() - 1 {
                    self.cursor_pos.1 -= 1;
                }
            }
        }
        let r = self.tasks[self.cursor_pos.0].remove(self.cursor_pos.1);
        if let Some(new_state) = new_state {
            self.tasks[new_state].push(r);
        }
    }

    fn submit_task(&mut self) {
        if !self.buffer.is_empty() {
            self.tasks[0].push(Task {
                content: self.buffer.clone(),
            });
            self.buffer = Vec::new();
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ])
            .split(area);

        let popup_area = Rect {
            x: 2 * area.width / 9,
            y: area.height / 2 - 3,
            width: 5 * area.width / 9,
            height: 6,
        };

        let left_title = Title::from(" TODO ".bold());
        let left_instructions = Title::from(Line::from(vec![
            " Move to:".into(),
            "  TODO <1>".blue().bold(),
            "  DOING <2>".light_red().bold(),
            "  DONE <3> ".green().bold(),
        ]));
        let mut left_block = Block::default()
            .title(left_title.alignment(Alignment::Center))
            .title(
                left_instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::ROUNDED);

        let center_title = Title::from(" Doing ".bold());
        let center_instructions = Title::from(Line::from(vec![
            " New entry ".into(),
            "<n>".blue().bold(),
            "  Delete entry ".into(),
            "<d> ".blue().bold(),
        ]));
        let mut center_block = Block::default()
            .title(center_title.alignment(Alignment::Center))
            .title(
                center_instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::ROUNDED);

        let right_title = Title::from(" Done ".bold());
        let right_instructions =
            Title::from(Line::from(vec![" Quit".into(), " <q> ".red().bold()]));
        let mut right_block = Block::default()
            .title(right_title.alignment(Alignment::Center))
            .title(
                right_instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::ROUNDED);

        match self.cursor_pos.0 {
            0 => {
                left_block = left_block.yellow();
            }
            1 => {
                center_block = center_block.yellow();
            }
            2 => {
                right_block = right_block.yellow();
            }
            _ => {}
        }

        Paragraph::new(
            self.tasks[0]
                .iter()
                .enumerate()
                .map(|(index, task)| {
                    if self.cursor_pos == (0, index) {
                        Line::from(task.content.clone().into_iter().collect::<String>())
                            .bold()
                            .light_red()
                    } else {
                        Line::from(task.content.clone().into_iter().collect::<String>()).white()
                    }
                })
                .collect::<Vec<_>>(),
        )
        .centered()
        .block(left_block)
        .render(layout[0], buf);

        Paragraph::new(
            self.tasks[1]
                .iter()
                .enumerate()
                .map(|(index, task)| {
                    if self.cursor_pos == (1, index) {
                        Line::from(task.content.clone().into_iter().collect::<String>())
                            .bold()
                            .light_red()
                    } else {
                        Line::from(task.content.clone().into_iter().collect::<String>()).white()
                    }
                })
                .collect::<Vec<_>>(),
        )
        .centered()
        .block(center_block)
        .render(layout[1], buf);

        Paragraph::new(
            self.tasks[2]
                .iter()
                .enumerate()
                .map(|(index, task)| {
                    if self.cursor_pos == (2, index) {
                        Line::from(task.content.clone().into_iter().collect::<String>())
                            .bold()
                            .light_red()
                    } else {
                        Line::from(task.content.clone().into_iter().collect::<String>()).white()
                    }
                })
                .collect::<Vec<_>>(),
        )
        .centered()
        .block(right_block)
        .render(layout[2], buf);

        if self.state == AppState::CreateTask {
            Clear.render(popup_area, buf);
            let popup_block = Block::default()
                .title(Title::from(" New Task ".bold().yellow()).alignment(Alignment::Center))
                .title(
                    Title::from(" Add <Enter> ".bold().light_red())
                        .alignment(Alignment::Left)
                        .position(Position::Bottom),
                )
                .title(
                    Title::from(" Close <Esc> ".bold().light_red())
                        .alignment(Alignment::Right)
                        .position(Position::Bottom),
                )
                .borders(Borders::ALL)
                .border_set(border::DOUBLE)
                .yellow();

            let popup_text = Text::from(vec![
                Line::from("Enter name"),
                Line::from(""),
                Line::from(
                    self.buffer
                        .clone()
                        .into_iter()
                        .collect::<String>()
                        .bold()
                        .white(),
                ),
            ]);

            Paragraph::new(popup_text)
                .centered()
                .block(popup_block)
                .render(popup_area, buf);
        }
    }
}

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let app_result = App::default().run(&mut terminal);
    tui::restore()?;
    app_result
}
