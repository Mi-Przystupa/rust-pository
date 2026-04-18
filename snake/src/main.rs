use rand::prelude::*;

use std::io::{self, Write, Stdout};
use std::vec::{Vec};
use crossterm::event::KeyModifiers;
use crossterm::{
    ExecutableCommand, QueueableCommand, 
    terminal, cursor, style::{self, Stylize},
    event::{read, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode}
};

const WIDTH: usize = 50;
const HEIGHT: usize = 25;

#[derive(Debug)]
struct Snake<'a> {
    body: &'a[(usize, usize, bool)],
    length: usize
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
    let mut i: usize = 10;
    let mut j: usize = 10;

    let mut grid = create_grid(WIDTH, HEIGHT);
    let mut rng = rand::rng();

    let mut step: usize = 0;
    while let Ok(event) = read() {
        let Some(event) = event.as_key_press_event() else {
            continue;
        };

        let modifier = match event.modifiers {
            KeyModifiers::NONE => "".to_string(),
            _ => format!("{:}+", event.modifiers),
        };

        sio.execute(terminal::Clear(terminal::ClearType::All))?;
        let i_old = i;
        let j_old = j;
        i = update_psn(i, event.code, KeyCode::Right, KeyCode::Left);
        let boundary= 1;
        i = stay_in_boundary(i, boundary, WIDTH - boundary ); 
        j = update_psn(j, event.code, KeyCode::Down, KeyCode::Up);
        j = stay_in_boundary(j, boundary, HEIGHT - boundary);
        
        let i_old = stay_in_boundary(i_old, boundary, WIDTH - boundary ); 
        let j_old = stay_in_boundary(j_old, boundary, HEIGHT - boundary ); 

        if grid[j][i] == World::Fruit {

        }
        grid[j_old][i_old] = World::Empty;
        grid[j][i] = World::Player;


        step = step + 1;
        if step > 10 {
            let y: usize = rng.random_range(boundary..HEIGHT - boundary);
            let x: usize = rng.random_range(boundary..WIDTH - boundary);
            grid[y][x] = World::Fruit;
            step = 0;
        }


        draw_world(sio, &grid);
        println!("Key pressed: {modifier}{code}\r", code = event.code);
        println!("i value: {i}", i = i);
        if event.code == KeyCode::Esc {
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
