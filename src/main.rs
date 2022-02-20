use std::{
    io,
    time::Duration,
    error::Error, sync::mpsc, thread
};

use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand
};
use invaders::{frame::{self, new_frame}, render};

// use rusty_audio::Audio;

fn main() -> Result<(), Box<dyn Error>> {
    // let mut audio = Audio::new();
    // audio.add("explode", "../sound/explode.wav");
    // audio.add("lose", "../sound/lose.wav");
    // audio.add("move", "../sound/move.wav");
    // audio.add("pew", "../sound/pew.wav");
    // audio.add("startup", "../sound/startup.wav");
    // audio.add("win", "../sound/win.wav");
    // audio.play("win");

    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render loop in a separate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    // Game loop
    'gameloop: loop {
        // Per-frame init
        let curr_frame = new_frame();
        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        // audio.ploy("lose");
                        break 'gameloop;
                    },
                    _ => {}
                }
            }
        }
        // Draw & render
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));
    }

    // Cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    // audio.wait();
    stdout.execute(Show).unwrap();
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
