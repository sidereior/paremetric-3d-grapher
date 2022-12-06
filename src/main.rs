fn main() {
    use graplot::Plot3D;
    //add ability to change the bounds of the graph
    //you can't do 3 cos(&t&) because you have to explicetly identify the multiplicaitona dna division that takes place
    use std::io::{stdin,stdout,Write};
    let mut foft=String::new();
    print!("Please enter the x component of some vector-valued function in the form (x(&t&))  ex: (3cos(&t&)): ");
    let _=stdout().flush();
    stdin().read_line(&mut foft).expect("You did not enter a correct vector-valued function, try again");
    if let Some('\n')=foft.chars().next_back() {
        foft.pop();
    }
    if let Some('\r')=foft.chars().next_back() {
        foft.pop();
    }
    let mut goft=String::new();
    print!("Please enter the y component of some vector-valued function in the form (y(&t&))  ex: (5sin(&t&)): ");
    let _=stdout().flush();
    stdin().read_line(&mut goft).expect("You did not enter a correct vector-valued function, try again");
    if let Some('\n')=goft.chars().next_back() {
        
        goft.pop();
    }
    if let Some('\r')=goft.chars().next_back() {
        goft.pop();
    }
    let mut hoft=String::new();
    print!("Please enter the z component of some vector-valued function in the form (z(&t&))  ex: (2sin(&t&)): ");
    let _=stdout().flush();
    stdin().read_line(&mut hoft).expect("You did not enter a correct vector-valued function, try again");
    if let Some('\n')=hoft.chars().next_back() {
        hoft.pop();
    }
    if let Some('\r')=hoft.chars().next_back() {
        hoft.pop();
    }

    //set vales for xs
    let mut xs = [0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.];
    let mut ys = [0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.];
    let mut zs = [0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.];
    //now need to add user input to specify the bounds of the graph
    //and need to add ability to set bounds between
    for n in 0..14 {
        let mut m = n as i64;
        m = m - 6;
        let mut newfoft=foft.clone();
        let mut newgoft=goft.clone();
        let mut newhoft=hoft.clone();
        newfoft = newfoft.replace("&t&", &m.to_string());
        newgoft = newgoft.replace("&t&", &m.to_string());
        newhoft = newhoft.replace("&t&", &m.to_string());
        let myx = meval::eval_str(newfoft).unwrap();
        let myy = meval::eval_str(newgoft).unwrap();
        let myz = meval::eval_str(newhoft).unwrap();
        xs[n]=myx as f64;
        ys[n]=myy as f64;
        zs[n]=myz as f64;
        //why does cos or sin or tan not work
    }
    let plot = Plot3D::new((xs, ys, zs, "r-o"));
    plot.show();
}

/*
fn leanbender(i: &mut usize, x: &mut [usize]) {
    x[*i] += 1;
    *i += 1;
}
*/