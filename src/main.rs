use simptim::{exit, Exit, SysTime};
use std::io::{self, StdoutLock, Write};
pub use std::process::ExitCode;

use termion::{
    clear, cursor,
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
    style,
};

fn main() -> Exit {
    let stdout = io::stdout();
    let mut stdout = match stdout.lock().into_raw_mode() {
        Ok(ok) => ok,
        Err(err) => return exit(err),
    };
    let systime = SysTime::new();
    let mut size = (0u16, 0u16);
    let mut events = termion::async_stdin().keys();
    loop {
        if let Some(ev) = events.next() {
            match ev {
                Ok(Key::Esc | Key::Ctrl('c') | Key::Ctrl('d')) => break,
                Err(err) => return exit(err),
                _ => {}
            }
        }
        size = termion::terminal_size().unwrap_or(size);
        if let Err(err) = run(&mut stdout, &systime, size) {
            return exit(err);
        }
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    Exit::SUCCESS
}

fn run(
    stdout: &mut RawTerminal<StdoutLock<'_>>,
    systime: &SysTime,
    size: (u16, u16),
) -> simptim::Result {
    write!(stdout, "{}{}", clear::All, cursor::Hide)?;

    let (col, row) = size;
    let tm = systime.elapsed_time()?.to_string();
    let tm_size = tm.chars().count() as u16;
    {
        let row = row / 2;
        let col = (col / 2) - (tm_size / 2);
        write!(
            stdout,
            "{}{}{tm}{}{}",
            cursor::Goto(col, row),
            style::Bold,
            style::Reset,
            cursor::Goto(col, row)
        )?;
    }

    stdout.flush().map_err(From::from)
}
