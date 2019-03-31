fn main() {
    let data = ["abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab"];
    println!("{} ", data[0].chars().nth(0).unwrap());
    println!("{}", data[0].len());

    if data[0].chars().nth(0).unwrap() == data[0].chars().nth(0).unwrap(){
        println!("si funciona");
    }
    for a in data.iter(){
        println!("{}", a);
    }
}
