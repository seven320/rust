fn main(){
    let x = 12; //default i32
    let a = 12u8;
    let b = 4.3; //default f64
    let c = 4.3f32;
    let bv = true;
    let t = (13,false);
    let sentence = "hello world!";
    println!(
        "{} {} {} {} {} {} {} {}",
        x, a, b, c, bv, t.0,t.1, sentence
    );
}