

struct Vector {
    x: f64,
    y: f64,
    z: f64,
    x_last: Vec<f64>,
    y_last: Vec<f64>,
    z_last: Vec<f64>,
}

fn main(){
    let mut v = Vector {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        x_last: Vec::new(),
        y_last: Vec::new(),
        z_last: Vec::new(),
    };
    let mut i = 0;

    loop {
        let user_input = 1;
        //stablize the vector keep to 0
        println!("x:{}", v.x);
        println!("x:{}", v.x-1.0);
    }

    println!("Hello, world!");
}