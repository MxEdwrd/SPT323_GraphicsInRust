//GraphicsWithRust - Moving Circle
//Max Edward | 3/21/23

use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions}; //Minifb library for 2D graphics.
                                                                  //Key: maps keyboard input to check for keys pressed
                                                                  //MouseMode: specifies how mouse cursor should be treated (visible, hidden, locked, etc)
                                                                  //MouseButton: detects mouse button input
                                                                  //Window: window on users screen, provides methods for creating, updating, closing window
                                                                  //WindowOptions: configuration for window settings

//Constructor for circle graphic
struct Circle {
    x: f32,      //x position in window
    y: f32,      //y position in window
    radius: f32, //radius of circle graphic
    color: u32,  //color of circle graphic
}

//Main function
fn main() {
    let width = 640; //Defines window width (pixels)
    let height = 480; //Defines window height (pixels)
    let background_color = 0x0DECF2; //Defines window background color (hexidecimal)
    
    //Creates vector for initializing window with values of each pixel being 0
    let mut buffer = vec![0; width * height];
    

    //Defines variable for circle data
    let mut circle = Circle {
        x: width as f32 / 2.0, //Defines x position as f32, then divides value by 2
        y: height as f32 / 2.0, //Defines y position as f32, then divides value by 2
        radius: 50.0,          //Defines circle radius (diameter is * 2)
        color: 0xFFA500,       //Defines color of circle in hexidecimal
    };

    //Defines variable for window data
    let mut window = Window::new(
        "Moving Circle",          //Name of window
        width,                    //Sets width as width variable
        height,                   //Sets height as height variable
        WindowOptions::default(), //Sets the window options to defaults
    )
    
    //Panic method to handle errors when working with result object
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    //While loop for when the window is open and the escape key is not pressed
    while window.is_open() && !window.is_key_down(Key::Escape) {
        
        //Sets all window pixels to defined background color
        buffer.iter_mut().for_each(|pixel| *pixel = background_color); 

        for x in 0..width { //Initialize circle
            //Generates circle with given center point (circle.x, circle.y) and defines width, and color using variables
            for y in 0..height {
                let dx = x as f32 - circle.x;
                let dy = y as f32 - circle.y;
                let distance = (dx * dx + dy * dy).sqrt();
                if distance < circle.radius {
                    let index = y * width + x;
                    buffer[index] = circle.color;
                }
            }
        }

        //Updates window with new pixels
        window.update_with_buffer(&buffer, width, height).unwrap();

        //Code for mouse position
        //Gets current position, discards if outside window area
        let mouse = window.get_mouse_pos(MouseMode::Discard); 
        
        //Calculate distance from cursor to center of circle
        if let Some((x, y)) = mouse {
            let dx = x as f32 - circle.x;
            let dy = y as f32 - circle.y;
            let distance = (dx * dx + dy * dy).sqrt();
            
            //If center of circle and cursor are different & left mouse is pressed, move center of circle to match cursor
            //Cursor has to be touching somewhere inside circle
            if distance < circle.radius && window.get_mouse_down(MouseButton::Left) {
                circle.x = x as f32;
                circle.y = y as f32;
            }
        }
    }
}
