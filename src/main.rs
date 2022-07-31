fn main() {
    // 4) set temperature and scale to use with Scale enums
    let cels = Scale::Cels(32.0);
    let fahr = Scale::Fahr(89.6);

    // 5) create a mutable Convert object with a cels scale property
    let mut fin = Convert { scale: cels, };

    // 6) call the implemented function to convert temperature and println
    fin.call();

    // 7) mutate the Convert object to be of a fahr Scale type now...
    fin = Convert { scale: fahr, };

    // 8) call the implemented function to convert temperature and println
    fin.call();
}
enum Scale { // 1) create an enum to set "options" for scale (c or f). This creates a "custom type".
    Cels (f32),
    Fahr (f32)
}
struct Convert { // 2) create a struct to use enum Scale Type - this is just to show one example of how they can be used together.
    scale: Scale, // The data type of the "scale" property will be a Scale
}

impl Convert { // 3) implement a function to the Convert struct
    fn call(&self) { // the name of the function is call and it uses itself (aka '&self')
        match self.scale { // use 'match' on the 'scale' property of 'self' to create different cases for each condition.
            Scale::Cels(temp) => { // 3a) set a case for Scale::Cels type - and use "temp" to use in following code...
                let f_temp = (temp * 9.0/5.0) + 32.0;
                println!("{} degrees celcius is {} degrees fahrenheit", temp, f_temp )
            }
            Scale::Fahr(temp) => { // 3b) set a case for Scale::Fahr type - and use "temp" to use in following code...
                let c_temp = (temp - 32.0) * 5.0/9.0;
                println!("{} degrees fahrenheit is {} degrees celsius", temp, c_temp )
            }
        }
    }
}