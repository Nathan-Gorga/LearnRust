fn main() {
    //loop keyword

    // loop{//infinite loop
    //     println!("Hello world")
    // }



    let mut counter: i32 = 0;


    let result = loop{
        counter += 1;

        if counter == 10{
            break counter -100;
        }
    };

    println!("The result is {result}");




    let mut count = 0;

    'counting_up: loop{
        println!("count = {count}");
        let mut remaining = 10;

        loop{
            println!("remaining = {remaining}");
            if remaining == 9{
                break;
            }
            if count == 2{
                break 'counting_up;
            }
            remaining -= 1;
        }
        count+=1;
    }


    let mut number = 3;
    while number != 0{
        println!("{number}");
        number-= 1;
    }



    let a = [1,2,3,4,5,6];

    for element in a {
        println!("{element}");
    }

}
