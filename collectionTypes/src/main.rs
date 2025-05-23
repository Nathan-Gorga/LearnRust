fn main() {
    //vectors

    // let _v: Vec<i32> = Vec::new();
    // let _the_vec: Vec<i32> = vec![1,2,3];



    // let mut _the_numbers_vec: Vec<i32> = Vec::new();
    // _the_numbers_vec.push(5);
    // _the_numbers_vec.push(6);
    // _the_numbers_vec.push(7);
    // _the_numbers_vec.push(8);
    // _the_numbers_vec.push(9);

    // println!("{:?}",_the_numbers_vec);

    let _v: Vec<i32> = vec![1,2,3,4,5];


    let third: &i32 = &_v[2];//Direct indexing

    println!("the third element is {third}");





    let third: Option<&i32> = _v.get(3);
    match third{
        Some(third) => println!("The third element for a GET method is {third}"),

        None => println!("There is no third element."),
    }




    //utf8





    //hash maps





}
