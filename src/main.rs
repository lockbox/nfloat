#![feature(option_result_unwrap_unchecked)]
use clap::{App, Arg};

#[derive(Debug)]
struct FloatSize {
    s: u32,
    exp: u32,
    frac: u32,
}

impl FloatSize {
    fn new(exp: u32, frac: u32) -> FloatSize {
        FloatSize { s: 1, exp, frac }
    }

    fn bias(&self) -> u32 {
        2_u32.pow(self.exp - 1) - 1
    }
}

#[derive(Debug)]
struct FloatingNumber {
    sign: u32,
    exponent: u64,
    fraction: u64,
}

impl FloatingNumber {
    fn new(sign: u32, exponent: u64, fraction: u64) -> FloatingNumber {
        FloatingNumber {
            sign,
            exponent,
            fraction,
        }
    }
}

fn print_float(val: u64) {
    let msb: u32 = num_bits(val);
    println!("+{:=<1$}+", "", msb as usize * 3);
}

fn num_bits(val: u64) -> u32 {
    assert!(val > 0);
    // the following is actually safe bc of the above assert!
    unsafe {
        *(64..=0)
            .collect::<Vec<u32>>()
            .iter()
            .find(|&&i| ((val >> i) & 1) == 1)
            .unwrap_unchecked()
    }
}

fn dec_to_float(fsize: FloatSize, val: f64) {
    let bias = fsize.bias();
    //let msb = get_msb(val);
}

fn main() {
    // from decimal, also should do fraction + binary form
    let from_decimal = App::new("dec")
        .about("print an arbitrary size IEEE754 float into various forms")
        .arg(
            Arg::new("value")
                .short('n')
                .long("val")
                .about("decimal number to parse (ex 4.3)")
                .takes_value(true)
                .required(true),
        );

    let matches = App::new("nFloat")
        .version("0.1")
        .about("sometimes you need to know what a float of arbitrary bit size looks like")
        .subcommands([from_decimal])
        .arg(
            Arg::new("exp-bits")
                .short('e')
                .long("exp-bits")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("frac-bits")
                .short('f')
                .long("frac-bits")
                .about("number of bits in the fraction portion")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let exp: u32 = matches.value_of_t_or_exit("exp-bits");
    let frac: u32 = matches.value_of_t_or_exit("frac-bits");

    // using 64 bit data types
    assert!(exp < 63);
    assert!(frac < 63);
    let fsize = FloatSize::new(exp, frac);

    match matches.subcommand() {
        Some(("dec", dec_matches)) => {
            let value: f64 = dec_matches.value_of_t_or_exit("value");
            print!("converting {} from decimal", value);
            println!(
                " to float with 1 sign bit, {} exp bits, and {} frac bits",
                fsize.exp, fsize.frac
            );
            dec_to_float(fsize, value);
        }
        None => println!("No subcommand used, see '--help'"),
        _ => unreachable!(),
    }
}
