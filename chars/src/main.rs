use std::str;

fn main() {
    let tao = str::from_utf8(&[0xe9u8, 0x81u8, 0x93u8]).unwrap();
    assert_eq!("道", tao);
    assert_eq!("道", String::from("\u{9053}"));
    let unicode_x = 0x9053;  // 码位
    let utf_x_hex = 0xe98193;  // utf-8 十六进制
    let utf_x_bin = 0b111010011000000110010011;  // utf-8 二进制
    println!("unicode_x: {:b}", unicode_x);
    println!("utf_x_hex: {:b}", utf_x_hex);
    println!("utf_x_bin: 0x{:x}", utf_x_bin);

    let tao = '道';
    let tao_u32 = tao as u32;  // char 是有效u32整数
    assert_eq!(36947, tao_u32);
    println!("U+{:x}", tao_u32);
    println!("{}", tao.escape_unicode());  // 获取标量值 \u{9053}
    assert_eq!(char::from(65), 'A');
    assert_eq!(std::char::from_u32(0x9053), Some('道'));
    assert_eq!(std::char::from_u32(36947), Some('道'));
    assert_eq!(std::char::from_u32(901010101), None);

    let mut b = [0; 3];
    let tao_str = tao.encode_utf8(&mut b);
    assert_eq!("道", tao_str);
    assert_eq!(3, tao.len_utf8());

    assert_eq!(true, 'f'.is_digit(16));       // 判断是不是十六进制
    assert_eq!(Some(15), 'f'.to_digit(16));   // 转换为十六进制
    assert!('a'.is_lowercase());
    assert!(!'道'.is_lowercase());
    assert!(!'a'.is_uppercase());
    assert!('A'.is_uppercase());
    assert!(!'中'.is_uppercase());
    assert_eq!('i', 'I'.to_ascii_lowercase());
    assert_eq!('B', 'b'.to_ascii_uppercase());
    assert!(' '.is_whitespace());
    assert!('\u{A0}'.is_whitespace());
    assert!(!'越'.is_whitespace());
    assert!('a'.is_alphabetic());
    assert!('晶'.is_alphabetic());
    assert!(!'1'.is_alphabetic());
    assert!('7'.is_alphanumeric());
    assert!('K'.is_alphanumeric());
    assert!('藏'.is_alphanumeric());
    assert!('¾'.is_alphanumeric());
    assert!(!'q'.is_control());
    assert!('7'.is_numeric());
    assert!(!'藏'.is_numeric());
    println!("{}", '\r'.escape_default());

    let mut a = String::from("fooa");
    println!("{:p}", a.as_ptr());  // 堆指针
    println!("{:p}", &a);  // 栈上指针
    assert_eq!(a.len(), 4);
    a.reserve(10);
    assert_eq!(a.capacity(), 14);

    let string: String = String::new();
    assert_eq!("", string);
    let string: String = String::from("hello rust");
    assert_eq!("hello rust", string);
    let string: String = String::with_capacity(20);
    assert_eq!("", string);
    let str: &'static str = "the tao of rust";
    let string: String = str.chars().filter(|c| !c.is_whitespace()).collect();
    assert_eq!("thetaoofrust", string);
    let string: String = str.to_owned();
    assert_eq!("the tao of rust", string);
    let string: String = str.to_string();
    let str: &str = &string[11..15];
    assert_eq!("rust", str);
}