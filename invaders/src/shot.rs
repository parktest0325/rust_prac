use std::time::Duration;
use rusty_time::Timer;
use crate::frame::{Drawable, Frame};

pub struct Shot {
    pub x: usize,
    pub y: usize,
    pub exploding: bool,
    timer: Timer,
}

impl Shot {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            exploding: false,
            timer: Timer::new(Duration::from_millis(50)),
        }
    }
   // 총알의 타이머가 delta 시간만큼 계속 업데이트 되면가 지정한 시간이 지나면 위치가 1씩 감소한다.
    pub fn update(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if self.timer.finished() && !self.exploding {
            if self.y > 0 {
                self.y -= 1;
            } 
            self.timer.reset();
        }
    }

    pub fn explode(&mut self) {
        self.exploding = true;
        self.timer = Timer::new(Duration::from_millis(250));
    }

    pub fn dead(&self) -> bool {
        (self.exploding && self.timer.finished()) || (self.y == 0)
    }
}

impl Drawable for Shot {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = if self.exploding {"*"} else {"|"}
    }
}