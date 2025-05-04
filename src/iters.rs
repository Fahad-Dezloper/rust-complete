pub fn iters(){
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    while let Some(val) = v1_iter.next(){
        println!("{}", val);
    }

    // other  way
    // for val in v1_iter {
    //     println!("Got: {val}");
    // }

    println!("{:?}", v1);
}