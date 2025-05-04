use std::collections::HashMap;


fn group_values_by_keys(vec: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut hm = HashMap::new();
    for (key, value) in vec {
        hm.insert(key, value);
    }
    return hm;
}


pub fn hash_map_que(){
    let input_vec = vec![(String::from("Fahad"), 25), (String::from("Raman"), 25)];
    let hm = group_values_by_keys(input_vec);

    println!("{:?}", hm);
}