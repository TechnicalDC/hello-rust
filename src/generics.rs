struct Rect<I>{
    width: I,
    length: I
}

//impl<I> Rect<I> {
//    // J generic type is scoped to the implemented method only
//    fn area<J>(&self) -> I {
//        self.length + self.width
//    }
//}

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

