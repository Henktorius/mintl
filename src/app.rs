use std::{env, fs::OpenOptions, io, io::Write};

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

use crate::styles::Styles;
use crate::tui;

#[derive(Debug, Clone)]
pub struct Task {
    pub content: Vec<char>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AppState {
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
    pub tasks: Vec<Vec<Task>>,
    pub state: AppState,
    pub exit: bool,
    pub buffer: Vec<char>,
    pub cursor_pos: (usize, usize),
    pub styles: Styles,
}

impl App {
    pub fn default() -> Self {
        App {
            tasks: vec![Vec::new(), Vec::new(), Vec::new()],
            state: AppState::default(),
            exit: false,
            buffer: Vec::new(),
            cursor_pos: (0, 0),
            styles: Styles::default(),
        }
    }

    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        let save_content = self.tasks_to_chars();
        self.save_and_exit(save_content)
    }

    fn save_and_exit(&mut self, content: Vec<u8>) -> io::Result<()> {
        match env::current_dir() {
            Ok(env_path) => {
                match OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .append(false)
                    .open(env_path.join(".mintl"))
                {
                    Ok(mut file) => {
                        if let Err(e) = file.write(&content) {
                            eprintln!("Failed to write to file: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to open or create save file: {}", e);
                    }
                };
            }
            Err(e) => {
                eprintln!("Failed to find working directory: {}", e)
            }
        }
        Ok(())
    }

    fn tasks_to_chars(&mut self) -> Vec<u8> {
        let mut r: Vec<u8> = Vec::new();
        for task in &self.tasks {
            task.iter().for_each(|t| {
                t.content.iter().for_each(|c| r.push(*c as u8));
                r.extend_from_slice("\t".as_bytes());
            });
            r.pop();
            r.extend_from_slice("\n".as_bytes());
        }
        r.pop();
        r
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
                KeyCode::Char('h') | KeyCode::Left => self.move_left(),
                KeyCode::Char('l') | KeyCode::Right => self.move_right(),
                KeyCode::Char('k') | KeyCode::Up => self.move_up(),
                KeyCode::Char('j') | KeyCode::Down => self.move_down(),
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
        if self.tasks[self.cursor_pos.0].is_empty() || (new_state.is_some() && self.cursor_pos.0 == new_state.unwrap()) {
            return;
        }
        let r = self.tasks[self.cursor_pos.0].remove(self.cursor_pos.1);
        //match self.tasks[self.cursor_pos.0].len() {
        //    0 => {
        //        return;
        //    }
        //    1 => {
        //        self.cursor_pos.1 = 0;
        //    }
        //    _ => {
        //        if self.cursor_pos.1 == self.tasks[self.cursor_pos.0].len() {
        //            self.cursor_pos.1 -= 1;
        //        }
        //    }
        //}
        if self.cursor_pos.1 > 0 {
            self.cursor_pos.1 -= 1;
        }
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
        let global_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(100), Constraint::Length(3)])
            .split(area);

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(33),
                Constraint::Percentage(34),
                Constraint::Percentage(33),
            ])
            .split(global_layout[0]);

        let popup_area = Rect {
            x: 2 * area.width / 9,
            y: area.height / 2 - 3,
            width: 5 * area.width / 9,
            height: 6,
        };

        let left_title = Title::from(" TODO ".bold());
        let mut left_block = Block::default()
            .title(left_title.alignment(Alignment::Center))
            .borders(Borders::ALL)
            .border_set(border::ROUNDED);

        let center_title = Title::from(" Doing ".bold());
        let mut center_block = Block::default()
            .title(center_title.alignment(Alignment::Center))
            .borders(Borders::ALL)
            .border_set(border::ROUNDED);

        let right_title = Title::from(" Done ".bold());
        let mut right_block = Block::default()
            .title(right_title.alignment(Alignment::Center))
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

        let blocks = vec![left_block, center_block, right_block];

        for (i, block) in blocks.iter().enumerate() {
            Paragraph::new(
                self.tasks[i]
                    .iter()
                    .enumerate()
                    .map(|(index, task)| {
                        Line::styled(
                            task.content.clone().into_iter().collect::<String>(),
                            if self.cursor_pos == (i, index) {
                                self.styles.s_list_element_hl
                            } else {
                                self.styles.s_list_element
                            },
                        )
                    })
                    .collect::<Vec<_>>(),
            )
            .centered()
            .block(block.clone())
            .render(layout[i], buf);
        }

        Paragraph::new(Line::from(vec![
            Span::styled("New entry ", self.styles.s_instruction_tag),
            Span::styled("<n>", self.styles.s_new_entry_key),
            Span::styled("  Delete entry ", self.styles.s_instruction_tag),
            Span::styled("<d>", self.styles.s_delete_entry_key),
            Span::styled("  Move: ", self.styles.s_instruction_tag),
            Span::styled("<hjkl>", self.styles.s_move_key),
            Span::styled("  Change state: ", self.styles.s_instruction_tag),
            Span::styled("TODO <1>", self.styles.s_todo_key),
            Span::styled(" ", self.styles.s_instruction_tag),
            Span::styled("DOING <2>", self.styles.s_doing_key),
            Span::styled(" ", self.styles.s_instruction_tag),
            Span::styled("DONE <3>", self.styles.s_done_key),
            Span::styled("  Quit ", self.styles.s_instruction_tag),
            Span::styled("<q>", self.styles.s_quit_key),
        ]))
        .centered()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_set(border::ROUNDED),
        )
        .render(global_layout[1], buf);

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
