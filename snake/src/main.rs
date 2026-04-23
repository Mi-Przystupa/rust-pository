use rand::prelude::*;

use std::io::{self, Write, Stdout};
use std::vec::{Vec};
use std::{thread, time};
use crossterm::event::KeyModifiers;
use crossterm::{
    ExecutableCommand, QueueableCommand, 
    terminal, cursor, style::{self, Stylize},
    event::{read, KeyCode, poll},
    terminal::{disable_raw_mode, enable_raw_mode}
};

const WIDTH: usize = 50;
const HEIGHT: usize = 25;
const BOUNDARY: usize = 1;

#[derive(Debug)]
struct Snake {
    head: (usize, usize),
    body: Vec<(usize, usize)>,
    tail: (usize, usize),
}

impl Snake {
    fn move_body(&mut self) -> (usize, usize) {
        let mut new_positions:  Vec::<(usize, usize)> = Vec::new();
        new_positions.push(self.head);

        let l: usize = self.body.len();
        let ret: (usize, usize) =self.body[l];
        self.body.truncate(l);

        for old_position in self.body.iter() {
            let to_push: (usize, usize) = (old_position.0, old_position.1);
            new_positions.push(to_push);
        }
        self.body = new_positions;

        return ret

    }
    fn update_head(&mut self, code: KeyCode) -> (usize, usize) {
        let (i_old, j_old) = self.head;
        self.head.0 = update_psn(self.head.0, code, KeyCode::Right, KeyCode::Left);
        
        self.head.0 = stay_in_boundary(self.head.0, BOUNDARY, WIDTH - BOUNDARY ); 

        self.head.1 = update_psn(self.head.1, code, KeyCode::Down, KeyCode::Up);
        self.head.1 = stay_in_boundary(self.head.1, BOUNDARY, HEIGHT - BOUNDARY);
        
        let i_old = stay_in_boundary(i_old, BOUNDARY, WIDTH - BOUNDARY ); 
        let j_old = stay_in_boundary(j_old, BOUNDARY, HEIGHT - BOUNDARY ); 

        return (i_old, j_old)

    }
    fn update_position(&mut self, code: KeyCode) -> (usize, usize) {
        let l: usize = self.body.len();
        if l > 0 {
            self.tail = self.move_body();    
        } else {
            self.tail.0 = self.head.0;
            self.tail.1 = self.head.1;
        }

        return self.update_head(code)
        
    }

    fn grow_body(&mut self) {

       let mut to_add: Vec<(usize, usize)> = vec![self.tail];
       self.body.append(&mut to_add);
    }
}

#[derive(Debug, PartialEq, Eq)]
enum World {
    Player, // Red
    Fruit, // Blue
    Wall, // Magenta
    Empty, //clear
}

enum Color {
    Red,
    Blue,
    Magenta,
    //Green
}

fn write_coord(sio: &mut Stdout, x: usize, y: usize, color : Color) {
    match color {
        Color::Red => {sio 
          .queue(cursor::MoveTo(x.try_into().unwrap(),y.try_into().unwrap())).expect("")
          .queue(style::PrintStyledContent( "i".red())).expect("");
        },
        Color::Magenta => {sio 
          .queue(cursor::MoveTo(x.try_into().unwrap(),y.try_into().unwrap())).expect("")
          .queue(style::PrintStyledContent( "i".magenta())).expect("");
        },
        Color::Blue => {sio 
          .queue(cursor::MoveTo(x.try_into().unwrap(),y.try_into().unwrap())).expect("")
          .queue(style::PrintStyledContent( "i".blue())).expect("");
        },
    }
}
    
fn draw_world(sio: &mut Stdout, grid: &Vec<Vec<World>>) {
  for y in 0..HEIGHT {
    for x in 0..WIDTH {
       match grid[y][x] {
            World::Wall => write_coord(sio, x, y, Color::Magenta),   
            World::Player =>  write_coord(sio, x, y, Color::Red),   
            World::Fruit => write_coord(sio, x, y, Color::Blue),
            _ => (),
        }
    }
  }
}

fn update_psn(val: usize, code: KeyCode, incr_code: KeyCode, dec_code: KeyCode) -> usize {
    match code {
        c if c == incr_code => val + 1,
        c if c == dec_code => val - 1,
        _ => val,
    }
}

fn stay_in_boundary(val: usize, low: usize, high: usize) -> usize{
    if val <= low {
        low
    } else if val >= high {
        high - 1   
    } else {
        val
    }
}

fn create_grid(width: usize, height: usize) -> Vec<Vec<World>>{
    let mut grid = Vec::new(); //#[[World::Empty; width]; height];
    for y in 0..height {
        let mut row = Vec::new();
        for x in 0..width {
          if (y == 0 || y == HEIGHT - 1) || (x == 0 || x == WIDTH - 1) {
            // in this loop we are more efficient by not flushing the buffer.
            row.push(World::Wall);
          } else {
            row.push(World::Empty);
          }
        }
        grid.push(row);
    }

    grid
}


fn execute_program(sio: &mut Stdout) -> io::Result<()> {
    //let mut i: usize = 10;
    //let mut j: usize = 10;

    let mut grid = create_grid(WIDTH, HEIGHT);
    let mut rng = rand::rng();

    let mut step: usize = 0;
    let mut code: KeyCode  = KeyCode::Backspace;
    let mut modifier: String = String::new();
    let mut snake: Snake = Snake {
            head: (10, 10),
            body: vec![],
            tail: (10, 11), // for appending
    };

    let pause = time::Duration::from_millis(500);
    loop {
        
       if poll(pause)? {
            let Ok(event) = read() else { 
                panic!("Cannot read any more!")
            };

            if let Some(event) = event.as_key_press_event() {
                code = event.code;
                modifier = match event.modifiers {
                    KeyModifiers::NONE => "".to_string(),
                    _ => format!("{:}+", event.modifiers),
                };

            }         
        }
        
        sio.execute(terminal::Clear(terminal::ClearType::All))?;
        /*
        let (i_old, j_old) = snake.head;
        snake.head.0 = update_psn(snake.head.0, code, KeyCode::Right, KeyCode::Left);
        let boundary= 1;
        
        snake.head.0 = stay_in_boundary(snake.head.0, boundary, WIDTH - boundary ); 

        snake.head.1 = update_psn(snake.head.1, code, KeyCode::Down, KeyCode::Up);
        snake.head.1 = stay_in_boundary(snake.head.1, boundary, HEIGHT - boundary);
        
        let i_old = stay_in_boundary(i_old, boundary, WIDTH - boundary ); 
        let j_old = stay_in_boundary(j_old, boundary, HEIGHT - boundary ); 
        */
        let (i_old, j_old) = snake.update_position(code);
        grid[j_old][i_old] = World::Empty;
        grid[snake.head.1][snake.head.0] = World::Player;


        step = step + 1;
        if step > 10 {
            let y: usize = rng.random_range(BOUNDARY..HEIGHT - BOUNDARY);
            let x: usize = rng.random_range(BOUNDARY..WIDTH - BOUNDARY);
            grid[y][x] = World::Fruit;
            step = 0;
        }


        draw_world(sio, &grid);
        if code == KeyCode::Esc {
            break;
        }
        
        
    }
    Ok(())

}

fn main() -> io::Result<()> {
  let mut stdout = io::stdout();
  enable_raw_mode()?; 
  if let Err(e) = execute_program(&mut stdout) {
    println!("Error: {e:?}\r");
  }
  stdout.flush()?;
  disable_raw_mode()?;
  Ok(())
}
