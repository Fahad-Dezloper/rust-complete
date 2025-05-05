// into iters take there ownership so orginal colloection wont exist later

pub fn intoiters(){
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.into_iter();

    for val in v1_iter {
        println!("into iter {}", val);
    }
}