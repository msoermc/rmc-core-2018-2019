extern crate mml;

fn generate_uml() {
    let dest: String = concat!("target/doc/", env!("CARGO_PKG_NAME")).to_string();
    
    let _ = mml::src2both("src", dest.replace("-", "_").as_str());
}


fn main() {
    //generate_uml();
}