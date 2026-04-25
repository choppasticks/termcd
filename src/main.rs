use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute, terminal,
};
use std::io::{self, Write};

struct Buffer {
    text: String,
    cursor_x: usize,
    cursor_y: usize,
}

impl Buffer {
    fn new() -> Self {
        Self {
            text: String::new(),
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    fn lines(&self) -> Vec<&str> {
        self.text.split('\n').collect()
    }

    fn current_line_len(&self) -> usize {
        self.lines()
            .get(self.cursor_y)
            .map(|l| l.len())
            .unwrap_or(0)
    }

    fn index(&self) -> usize {
        let mut ind = 0;
        let lines = self.lines();
        for y in 0..self.cursor_y {
            ind += lines[y].len() + 1;
        }
        ind + self.cursor_x
    }

    fn insert(&mut self, c: char) {
        self.text.insert(self.index(), c);
        self.cursor_x += 1;
    }

    fn newline(&mut self) {
        self.text.insert(self.index(), '\n');
        self.cursor_x = 0;
        self.cursor_y += 1;
    }

    fn delete(&mut self) {
        if self.index() > 0 {
            self.text.remove(self.index() - 1);
            if self.cursor_x > 0 {
                self.cursor_x -= 1;
            } else {
                self.cursor_y -= 1;
                self.cursor_x = self.current_line_len();
            }
        }
    }

    fn move_left(&mut self) {
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
        }
    }

    fn move_right(&mut self) {
        if self.cursor_x < self.current_line_len() {
            self.cursor_x += 1;
        }
    }

    fn move_down(&mut self) {
        if self.cursor_y < self.lines().len() {
            self.cursor_y += 1;
        }
    }

    fn move_up(&mut self) {
        if self.cursor_y > 0 {
            self.cursor_y -= 1;
        }
    }
}

fn main() -> io::Result<()> {
    terminal::enable_raw_mode().unwrap();

    let mut stdout = std::io::stdout();

    execute!(stdout, terminal::EnterAlternateScreen, cursor::Show)?;

    let mut buffer = Buffer::new();

    loop {
        execute!(
            stdout,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(buffer.cursor_x as u16, buffer.cursor_y as u16)
        )?;
        write!(stdout, "{}", buffer.text)?;
        stdout.flush()?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => break,
                KeyCode::Char(c) => buffer.insert(c),
                KeyCode::Backspace => buffer.delete(),
                KeyCode::Enter => buffer.newline(),
                KeyCode::Up => buffer.move_up(),
                KeyCode::Down => buffer.move_down(),
                KeyCode::Left => buffer.move_left(),
                KeyCode::Right => buffer.move_right(),
                _ => {}
            }
        }
    }

    execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
