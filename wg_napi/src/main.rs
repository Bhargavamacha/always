mod lib;
fn main(){
    let data = lib::wg_start();
    println!("{}", data);
}