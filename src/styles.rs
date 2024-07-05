use ratatui::style::{Color, Modifier, Style};

#[derive(Debug)]
pub struct Styles {
    pub s_quit_key: Style,
    pub s_new_entry_key: Style,
    pub s_delete_entry_key: Style,
    pub s_move_key: Style,
    pub s_instruction_tag: Style,
    pub s_todo_key: Style,
    pub s_doing_key: Style,
    pub s_done_key: Style,
    pub s_list_element: Style,
    pub s_list_element_hl: Style,
}

impl Default for Styles {
    fn default() -> Self {
        Styles {
            s_quit_key: Style::default()
                .fg(Color::Red)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),

            s_new_entry_key: Style::default()
                .fg(Color::LightBlue)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),

            s_delete_entry_key: Style::default()
                .fg(Color::LightBlue)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),

            s_move_key: Style::default()
                .fg(Color::LightBlue)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),

            s_instruction_tag: Style::default().fg(Color::White).bg(Color::Black),

            s_todo_key: Style::default()
                .fg(Color::LightBlue)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),

            s_doing_key: Style::default()
                .fg(Color::LightRed)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),

            s_done_key: Style::default()
                .fg(Color::Green)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),

            s_list_element: Style::default().fg(Color::White).bg(Color::Black),

            s_list_element_hl: Style::default()
                .fg(Color::LightRed)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),
        }
    }
}
