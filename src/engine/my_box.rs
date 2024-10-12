use std::ops::Deref;

#[derive(Debug)]
struct MyBox<T> {
    value: T,
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> MyBox<T> {
    fn new(value: T) -> MyBox<T> {
        MyBox {
            value,
        }
    }
}


pub fn run_my_box() {
    let my_box_value = MyBox::new("string");
    println!("{:?}", my_box_value);
}
