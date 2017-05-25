// #![allow(dead_code)]
// use piston_window::*;
// pub struct Gui {
// //  pub window: PistonWindow,
//   pub x:u32,
//   pub y:u32,
// }

// impl Gui {
//   pub fn new(x:u32,y:u32)->Self {

//     return Gui{x:x, y:y}
//   }
//   pub fn draw_rectangle(mut self,upd:Event,window:PistonWindow, x:f64, y:f64, color:[f32;4]){
//     window.draw_2d(&upd, |c, g| {
//                 clear([1.0; 4], g);
//                 rectangle(color, // red
//                           [0.0, 0.0, x, y],
//                           c.transform,
//                           g);
//             });
//   }
// }