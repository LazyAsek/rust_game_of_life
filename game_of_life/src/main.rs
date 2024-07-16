


use sdl2::{event::Event, pixels::Color, rect::Point, render::Canvas, video::Window, EventPump, Sdl, VideoSubsystem};

fn init_grid (cells: &u32, cell_size: &u32,border : &u32) -> u32{
    return cells * cell_size + (cells+1) * border; 
}

fn lines_coords (cells: u32, cell_size:u32,border : u32) -> Vec<u32>{
    let mut coords: Vec<u32> =vec![0;(cells+1).try_into().unwrap()];
    let mut cur: u32 = 0;
    for i in 0..coords.len(){
        coords[i] = cur;
        cur= cur + cell_size +border;
    }
    return coords;
}

fn draw_lines_vertical( mut canvas : Canvas<Window> , coords : &Vec<u32>,size : u32) -> Canvas<Window>{
    for i in 0..coords.len(){
        let start : Point = Point::new(coords[i] as i32, 0);
        let end : Point = Point::new(coords[i] as i32, size as i32);
        canvas.draw_line(start,end).expect("lines broken");
    }
    return canvas ; 
}

fn draw_lines_horizontal(mut canvas : Canvas<Window>,coords : &Vec<u32>,size : u32) -> Canvas<Window>{
    for i in 0..coords.len(){
        let start : Point = Point::new(0,coords[i] as i32);
        let end : Point = Point::new(size as i32,coords[i] as i32);
        canvas.draw_line(start,end).expect("lines broken");
    }
    return canvas ;   
}

fn main() {
    let sld_context: Sdl = sdl2::init().unwrap();
    let video_subsystem : VideoSubsystem = sld_context.video().unwrap();

    const CELLS : u32 = 10;
    const CELL_SIZE : u32 =50;
    const BORDER_WITDH : u32 =1;

    let size : u32 = init_grid(&CELLS,&CELL_SIZE,&BORDER_WITDH);

    let coords : Vec<u32> = lines_coords(CELLS,CELL_SIZE,BORDER_WITDH);
    println!("{}",size);
    
    // todo
    //map grid
    //draw rectangle
    //introduze rules

    let window : Window = video_subsystem.window("Game of life", size, size)
        .position_centered()
        .build()
        .unwrap();

    let mut _canvas: Canvas<Window> = window.into_canvas().build().unwrap();

    let mut event_pump:EventPump = sld_context.event_pump().unwrap();

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
        _canvas = draw_lines_vertical(_canvas, &coords, size);
        _canvas = draw_lines_horizontal(_canvas, &coords, size);

        _canvas.present();
    }
}

