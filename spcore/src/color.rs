pub struct rgb {
    r: u8,
    g: u8,
    b: u8,
}
pub struct rgba {
    r: u8,
    g: u8,
    b: u8,
    a: Option<u8>
}
pub struct hsl {
    h: f64,
    s: f64,
    l: f64,
}

pub trait to_rgba {
    fn to_rgba(&self) -> rgba;
}
pub trait to_rgb {
    fn to_rgb(&self) -> rgb;
    
}
pub trait to_hex {
    fn to_hex(&self) -> String;
}


impl to_hex for rgba {
    fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}{:02x}", self.r, self.g, self.b, self.a.unwrap())
    }
}
impl to_hex for rgb {
    fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}
impl to_rgb for rgba {
    fn to_rgb(&self) -> rgb {
        rgb {
            r: self.r,
            g: self.g,
            b: self.b,
        }
    }
}
impl to_rgba for rgb {
    fn to_rgba(&self) -> rgba {
        rgba {
            r: self.r,
            g: self.g,
            b: self.b,
            a: Some(100),
        }
    }
}



impl rgba {
    fn new(r: u8, g: u8, b: u8, a: Option<u8>) -> rgba {
        //check if the alpha value is valid 
        if let Some(a) = a {
            if a > 255 {
                panic!("Alpha value must be between 0 and 255")
            }
        }
        rgba {
            r,
            g,
            b,
            a,
        }
    }
    
}
impl rgb {
    fn new(r: u8, g: u8, b: u8) -> rgb {
        rgb {
            r,
            g,
            b,
        }
    }
    fn to_rgba(&self) -> rgba {
        rgba {
            r: self.r,
            g: self.g,
            b: self.b,
            a: Some(100),
        }
    }
}