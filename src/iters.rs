// iter() use borrowing

pub fn iters(){
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    // one way to iterate
    // while let Some(val) = v1_iter.next(){
    //     println!("{}", val);
    // }

    // other  way
    // for val in v1_iter {
    //     println!("Got: {val}");
    // }

    // iter with method
    let iter2 = v1_iter.map(|x| x + 1);
    for x in iter2 {
        println!("{}", x);
    }

    println!("{:?}", v1);
}