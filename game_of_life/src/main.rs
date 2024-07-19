


use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::{Point, Rect}, render::Canvas, video::Window, EventPump, Sdl, VideoSubsystem};
use std:: thread::sleep;
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

fn _print_2dgrid(grid : & Vec<Vec<u32>>){
    for i in 0..grid.len(){
        for j in 0..grid.len(){
            print!("{}",grid[i][j])
        }
        println!(",")
    }
}

fn init_starting_squares(mut grid : Vec<Vec<u32>>, mut starting : i32)-> Vec<Vec<u32>>{
    let n: i32 = grid.len().try_into().unwrap();
    let mut i: i32 = n / 2;
    let mut j: i32 = n / 2;
 

    grid[i as usize][j as usize] = 1;
    starting = starting -1;

    let diractions : [(i32,i32);4]  = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    let mut range: i32=1;
    let mut index: i32 = 0;

    while starting > 0  {
        for _ in 0..2{
            for _ in 0..range{
                j = j+ diractions[index as usize].0;
                i = i+ diractions[index as usize].1;

                if j >= 0 && j < n && i >= 0 && i < n && starting > 0{
                    starting = starting -1;
                    grid[i as usize][j as usize]=1;

                }
            }
            index = (index +1) % 4;
        }
        range = range +1;
     
    }
    return grid
}

fn check_around(grid :&Vec<Vec<u32>>,i : usize,j: usize) -> [u32;8]{
    // get cell and reutn value of n,e,s,w from cell

    let n : u32 ;
    if i == 0 {
        n = grid[grid.len()-1][j];
    }
    else{
        n = grid[i-1][j];
    }

    let ne: u32;
    if i == 0 {
        if j == grid.len()-1 {
            ne = grid[grid.len()-1][0];
        }
        else{
            ne = grid[grid.len()-1][j+1];
        }
    }
    else{
        if j == grid.len()-1 {
            ne = grid[i-1][0];
        }
        else{
            ne = grid[i-1][j+1]
        }
    }

    
    let nw: u32;
    if i == 0 {
        if j == 0 {
            nw = grid[grid.len()-1][grid.len()-1];
        }
        else{
            nw = grid[grid.len()-1][j-1];
        }
    }
    else{
        if j == 0 {
            nw = grid[i-1][grid.len()-1];
        }
        else{
            nw = grid[i-1][j-1]
        }
    }

    let  s : u32 ;
    if i == grid.len()-1 {
        s = grid[0][j];
    }
    else{
        s = grid[i+1][j];
    }

    
    let se: u32;
    if i == grid.len()-1 {
        if j == grid.len()-1 {
            se = grid[0][0];
        }
        else{
            se = grid[0][j+1];
        }
    }
    else{
        if j == grid.len()-1 {
            se = grid[i+1][0];
        }
        else{
            se = grid[i+1][j+1]
        }
    }

    let sw: u32;
    if i == grid.len()-1 {
        if j == 0 {
            sw = grid[0][grid.len()-1];
        }
        else{
            sw = grid[0][j-1];
        }
    }
    else{
        if j == 0 {
            sw = grid[i+1][grid.len()-1];
        }
        else{
            sw = grid[i+1][j-1]
        }
    }

    let  e : u32 ;
    if j == grid.len()-1 {
        e = grid[i][0];
    }
    else{
        e = grid[i][j+1];
    }

    let  w : u32 ;
    if j == 0 {
        w = grid[i][grid.len()-1];
    }
    else{
        w = grid[i][j-1];
    }

    return  [n,ne,e,se,s,sw,w,nw];
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

fn game_of_life(mut grid : Vec<Vec<u32>>) -> Vec<Vec<u32>>{
    let mut count : u32;

    for i in 0..grid.len()-1{
        for j in 0..grid.len()-1{
            count = check_around(&grid, i, j).iter().sum();

            if grid[i][j] == 1{
                if count <2 || count > 3{
                    grid[i][j] = 0;
                }
            }
            else{
                if count == 3{
                    grid[i][j] = 1;
                }
            }

        }

    }

    return grid;
}

fn langton_ant (mut grid : Vec<Vec<u32>>,mut i : usize,mut j:usize,mut diraction : u32) -> (Vec<Vec<u32>>,usize,usize,u32) {
    // 1 n 2 e 3 s 4 w
    if grid[i][j] == 1{
        diraction = diraction +1;
        if diraction == 5{
            diraction = 1;
        }
        grid[i][j] = 0;
    }
    else{
        if diraction == 1{
            diraction = 4;
        }
        else{
            diraction = diraction -1;
        }
        grid[i][j] = 1;
    }

    if diraction == 1{
        if i == 0{
            i = grid.len()-1;
        }
        else{
            i = i -1
        }
    }
    if diraction == 2{
        if j == grid.len()-1{
            j=0
        }
        else {
            j = j +1;
        }
    }
    if diraction == 3{
        if i == grid.len()-1{
            i = 0;
        }
        else{
            i = i +1;
        }
    }
    if diraction == 4{
        if j == 0{
            j = grid.len()-1;
        }
        else {
            j = j -1;
        }
    }
    return (grid,i,j,diraction);

}
fn main() {

    //todo
    //introduze rules

    let sld_context: Sdl = sdl2::init().unwrap();
    let video_subsystem : VideoSubsystem = sld_context.video().unwrap();

    const CELLS : u32 = 60;
    const CELL_SIZE : u32 =10;
    const BORDER_WITDH : u32 =1;
    const STARTING_SQUARES : i32 = 0;
    //iterations delay between each in miliseconds
    const INTERVAL : u64 = 10;

    let size : u32 = init_grid(&CELLS,&CELL_SIZE,&BORDER_WITDH);

    let coords : Vec<u32> = lines_coords(&CELLS,&CELL_SIZE,&BORDER_WITDH);
    
    if STARTING_SQUARES > (CELLS * CELLS)as i32  {
        panic!("Too many starting squares");
    }

    let mut grid : Vec<Vec<u32>> = grid(&CELLS);
    grid = init_starting_squares(grid, STARTING_SQUARES);
 


    let window : Window = video_subsystem.window("Game of life", size, size)
        .position_centered()
        .build()
        .unwrap();

    let mut _canvas: Canvas<Window> = window.into_canvas().build().unwrap();

    let mut event_pump:EventPump = sld_context.event_pump().unwrap();
    let mut next_time = Instant::now() + Duration::from_millis(INTERVAL);
    let mut gol : bool = false;
    let mut ant : bool = false;
    let mut i : usize = grid.len()/2;
    let mut j : usize = grid.len()/2;
    let mut values : (Vec<Vec<u32>>,usize,usize,u32);
    let mut diraction : u32 = 1;

    'running : loop{
        for event in event_pump.poll_iter(){
            match event{
                Event::Quit { .. } => {
                    break 'running;
                },
                Event::KeyDown {keycode : Some(Keycode::Space)  , ..} =>{
                    gol = true;
                },
                Event::KeyDown {keycode : Some(Keycode::A), .. } =>{
                    ant = true;
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

        //game of life line activate after pressing space
        if gol == true{
            grid = game_of_life(grid);
        }

        if ant == true{
            values = langton_ant(grid, i, j, diraction);
            grid = values.0;
            i = values.1;
            j = values.2;
            diraction = values.3;
        }


        _canvas.present();
        
        sleep(next_time - Instant::now());
        next_time += Duration::from_millis(INTERVAL);
    }


}

