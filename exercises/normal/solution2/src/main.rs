mod converter;

fn main() {
    let num_str = "1111(2)";
    let base_to: u32 = 15;
    let result = converter::convert_base(num_str, base_to);
    println!("{} -> {}", num_str, result);
}
