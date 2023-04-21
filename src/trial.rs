fn main(){
    use plot3d::{Plot, Surface, Color};

let data = vec![
    (0.0, 0.0, 0.0),
    (0.0, 1.0, 0.0),
    (1.0, 0.0, 0.0),
    (1.0, 1.0, 1.0),
];

let surface = Surface::new(data)
    .color(Color::new(1.0, 0.0, 0.0))
    .name("surface");

let plot = Plot::new().add_surface(surface);
plot.show();
}