struct Rect<I>{
    width: I,
    length: I
}

#[warn(unused_variables)]
pub fn test_generics() {
    let rect: Rect<i32> = Rect {
        width: 100,
        length: 50
    };
    println!("w: {}, l: {}", rect.width, rect.length);
}
//pub fn test_generics<T>(list: &[T]) {
    //dbg!(list);
//}

