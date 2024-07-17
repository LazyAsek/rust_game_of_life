


use sdl2::{event::Event, pixels::Color, rect::{Point, Rect}, render::Canvas, video::Window, EventPump, Sdl, VideoSubsystem};
use std::thread::sleep;
use std::time::{Duration,Instant};

fn init_grid (cells: &u32, cell_size: &u32,border : &u32) -> u32{
    return cells * cell_size + (cells+1) * border; 
}

fn lines_coords (cells: &u32, cell_size:&u32,border : &u32) -> Vec<u32>{
    let mut coords: Vec<u32> =vec![0;(cells+0).try_into().unwrap()];
    let mut cur: u32 = 0;
    for i in 0..coords.len(){
        coords[i] = cur;
        cur= cur + cell_size +border;
    }
    return coords;
}

fn grid(cells :&u32) -> Vec<Vec<u32>>{
    let grid : Vec<Vec<u32>> = vec![vec![0;(cells+0).try_into().unwrap()];(cells+0).try_into().unwrap()];
    return grid;
}

fn print_2dgrid(grid : & Vec<Vec<u32>>){
    for i in 0..grid.len(){
        for j in 0..grid.len(){
            print!("{}",grid[i][j])
        }
        println!(",")
    }
}

fn init_starting_squares(mut grid : Vec<Vec<u32>>, mut starting : i32)-> Vec<Vec<u32>>{
    let n : i32 = grid.len().try_into().unwrap();
    let mut i : i32 = n/2;
    let mut j : i32 = n/2;
    let mut range : i32 = 1;
    let mut pref : i32 = -1;
    
    grid[i as usize][j as usize] = 1;
    starting = starting - 1;
    while starting >0{

        let mut cur :i32 = range;
        while j >=0 && j < n-1 && cur !=0{
            if starting == 0{
                break
            }
            j=j +1*pref;
            cur = cur -1;
            grid[i as usize][j as usize] = 1;
            starting = starting -1;
        }

        cur = range;        
        while i >=0 && i < n-1 && cur !=0{
            if starting == 0{
                break
            }
            i=i +1*pref*-1;
            cur = cur -1;
            grid[i as usize][j as usize] = 1;
            starting = starting -1;

        }
        range = range + 1;
        pref = pref*-1;
    }

    return grid;
}

fn draw_lines_vertical( mut canvas : Canvas<Window> , coords : &Vec<u32>,size : u32, width : u32) -> Canvas<Window>{
    for i in 0..coords.len(){
        for j in 0..width{
            let start : Point = Point::new((coords[i]+j) as i32, 0);
            let end : Point = Point::new((coords[i]+j) as i32, size as i32);
            canvas.draw_line(start,end).expect("lines broken");
        }
    }
    return canvas ; 
}

fn draw_lines_horizontal(mut canvas : Canvas<Window>,coords : &Vec<u32>,size : u32, width : u32) -> Canvas<Window>{
    for i in 0..coords.len(){
        for j in 0..width{
            let start : Point = Point::new(0,(coords[i]+j) as i32);
            let end : Point = Point::new(size as i32,(coords[i]+j) as i32);
            canvas.draw_line(start,end).expect("lines broken");
        }
    }
    return canvas ;   
}

fn draw_rectangles(mut canvas : Canvas<Window>, grid : & Vec<Vec<u32>> , coords : &Vec<u32> ,cell_size: u32) -> Canvas<Window>{

    for i in 0..grid.len(){
        for j in 0..grid.len(){
            if grid[i][j] == 1{
                let x : i32 = coords[j] as i32;
                let y : i32 = coords[i] as i32;
                canvas.fill_rect(Rect::new(x,y,cell_size,cell_size)).expect("Rectangle draw error");
            }
        }
    }

    return canvas
}

fn main() {

    //todo
    //introduze rules

    let sld_context: Sdl = sdl2::init().unwrap();
    let video_subsystem : VideoSubsystem = sld_context.video().unwrap();

    const CELLS : u32 = 100;
    const CELL_SIZE : u32 =10;
    const BORDER_WITDH : u32 =1;
    const STARTING_SQUARES : i32 = 123;
    //iterations delay between each in milisecounds
    const INTERVAL : u64 = 1000;

    let size : u32 = init_grid(&CELLS,&CELL_SIZE,&BORDER_WITDH);

    let coords : Vec<u32> = lines_coords(&CELLS,&CELL_SIZE,&BORDER_WITDH);
    
    if STARTING_SQUARES > (CELLS * CELLS)as i32{
        panic!("Too many starting squares");
    }

    let mut grid : Vec<Vec<u32>> = grid(&CELLS);
    grid = init_starting_squares(grid, STARTING_SQUARES);
    print_2dgrid(&grid);

    let window : Window = video_subsystem.window("Game of life", size, size)
        .position_centered()
        .build()
        .unwrap();

    let mut _canvas: Canvas<Window> = window.into_canvas().build().unwrap();

    let mut event_pump:EventPump = sld_context.event_pump().unwrap();
    let mut next_time = Instant::now() + Duration::from_millis(INTERVAL);

    'running : loop{
        for event in event_pump.poll_iter(){
            match event{
                Event::Quit { .. } => {
                    break 'running;
                },
                _ => {}
            }
        }
        _canvas.set_draw_color(Color ::RGB(0,0,0,));
        _canvas.clear();

        _canvas.set_draw_color(Color ::RGB(40, 42,51));
        _canvas = draw_lines_vertical(_canvas, &coords, size, BORDER_WITDH);
        _canvas = draw_lines_horizontal(_canvas, &coords, size,BORDER_WITDH);
        
        _canvas.set_draw_color(Color::RGB(255,255,255));
        _canvas = draw_rectangles(_canvas, &grid, &coords, CELL_SIZE);

        _canvas.present();
        sleep(next_time - Instant::now());
        next_time += Duration::from_millis(INTERVAL);
    }
}

