use std::{error::Error, io, sync::mpsc, thread, time::Duration};

use crossterm::{cursor::{Hide, Show}, event::{self, Event, KeyCode}, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand};
use invaders::{frame::{self, new_frame, Drawable}, player::Player, render};
use rusty_audio::Audio;

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "explode.wav");
    audio.add("lose", "lose.wav");
    audio.add("move", "move.wav");
    audio.add("pew", "pew.wav");
    audio.add("startup", "startup.wav");
    audio.add("win", "win.wav");
    audio.play("startup");

    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen);
    stdout.execute(Hide);

    // Render loop in a separate thread
    // 실제 프로젝트에서는 crossbeam이 더 좋다. 
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



    // Game Loop
    let mut player = Player::new();
    'gameloop: loop {
        // Per-frame init
        let mut curr_frame = new_frame();

        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    },
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    _ => {},
                }
            }
        }

        // Draw & render
        // 어떤 에러가 발생해도 무시한다. unwrap이나 ? 를 사용하면 에러 발생 시 종료됨 
        player.draw(&mut curr_frame);
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));
    }

    // Cleanup
    // join을 하지 않으면 러스트가 사용하지 않는 채널을 미리 drop 해버린다. 
    // 그럼 메인스레드가 먼저 종료될수도 있음
    drop(render_tx);
    render_handle.join().unwrap();
    
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok({})
}
