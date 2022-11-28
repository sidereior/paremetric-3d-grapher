
fn main() {
    use graplot::Plot3D;
    use std::io::{stdin,stdout,Write};
    let mut foft=String::new();
    print!("Please enter the x component of some vector-valued function in the form (x(t))  ex: (3cos(t)): ");
    let _=stdout().flush();
    stdin().read_line(&mut foft).expect("You did not enter a correct vector-valued function, try again");
    if let Some('\n')=foft.chars().next_back() {
        foft.pop();
    }
    if let Some('\r')=foft.chars().next_back() {
        foft.pop();
    }
    let mut goft=String::new();
    print!("Please enter the y component of some vector-valued function in the form (y(t))  ex: (5sin(t)): ");
    let _=stdout().flush();
    stdin().read_line(&mut goft).expect("You did not enter a correct vector-valued function, try again");
    if let Some('\n')=goft.chars().next_back() {
        goft.pop();
    }
    if let Some('\r')=goft.chars().next_back() {
        goft.pop();
    }
    let mut hoft=String::new();
    print!("Please enter the z component of some vector-valued function in the form (z(t))  ex: (2sin(t)): ");
    let _=stdout().flush();
    stdin().read_line(&mut hoft).expect("You did not enter a correct vector-valued function, try again");
    if let Some('\n')=hoft.chars().next_back() {
        hoft.pop();
    }
    if let Some('\r')=hoft.chars().next_back() {
        hoft.pop();
    }

    let xs = [0.,1.,2.,3.,4.,5.,6.];
    let ys = [0.,1.,4.,9.,16.,25.,36.];
    let zs = [0.,1.,4.,9.,16.,25.,36.];
    let plot = Plot3D::new((xs, ys, zs, "r-o"));
    plot.show();
}


/* 
fn main() {
    let graph = Graph::new();
let input_node = graph.source(41);
let output_node = input_node.clone().map(|x| x * x).shared();
assert_eq!(1681, output_node.get());

let t = thread::spawn({
    let input_node = input_node.clone();
    let output_node = output_node.clone();
    move || {
        input_node.update(|n| {
            *n += 1;
            true
        });

        output_node.get()
    }
});

assert_eq!(1764, t.join().unwrap());

input_node.update(|n| {
    *n += 1;
    true
});

assert_eq!(1849, output_node.get());
}
*/


