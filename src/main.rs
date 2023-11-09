use std::io::stdin;

fn quadraticform(a: f64,b: f64,c: f64) {
    let w = (b.powi(2)) - (f64::from(4) * a * c);
    if w < 0.0 {
        println!("No answer.")
    } else {
        let nb = f64::from(0) - b;
        let srw = f64::sqrt(w);
        let ta = f64::from(2) * a;
        let sol1 = (nb - srw) / ta;
        let sol2 = (nb + srw) / ta;
        println!("Solution 1: {}", sol1);
        println!("Solution 2: {}", sol2);
    }
}

fn main() -> std::io::Result<()> { 
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();
    println!("Enter number 1 (a): ");
    stdin().read_line(&mut a)?;
    println!("Enter number 2 (b): ");
    stdin().read_line(&mut b)?;
    println!("Enter number 3 (c): ");
    stdin().read_line(&mut c)?;

    // Translate the inputs into f64
    //first into &str
    let stra: &str = a.as_str();
    let strb: &str = b.as_str();
    let strc: &str = c.as_str();


    //remove \n at end (messes up program)
    let mut charsa = stra.chars();
    charsa.next_back();
    let ia: f64 = charsa.as_str().parse().unwrap();

    let mut charsb = strb.chars();
    charsb.next_back();
    let ib: f64 = charsb.as_str().parse().unwrap();

    let mut charsc = strc.chars();
    charsc.next_back();
    let ic: f64 = charsc.as_str().parse().unwrap();


    // run the prog
    quadraticform(ia, ib, ic);

    Ok(())
}