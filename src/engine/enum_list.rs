#[derive(Debug)]
enum Something {
    Variant1,
    Variant2,
    Variant3,
}

fn run (){
    let mut something=Something::Variant1;
    let mut something=Something::Variant2;
    let something=Something::Variant3;

    print!("{:?}",something);

}