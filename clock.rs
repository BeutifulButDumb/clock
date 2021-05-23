use std::time::{Duration};
use std::thread::sleep;
use std::io::{stdin, stdout, Write};

fn read(input: &mut String){
    stdout().flush()
        .expect("failed to flush");
    stdin().read_line(input)
        .expect("failde to read");
}

fn main(){
    print!("{}[2J", 27 as char); // cleats terminal
    
    let mut hour = String::new();
    println!("Enter a hour");
    read( &mut hour);
    let hour: i32 = hour.trim().parse().unwrap();

    print!("{}[2J", 27 as char);

    let mut min = String::new();
    println!("Enter the min");
    read( &mut min);
    let min: i32 = min.trim().parse().unwrap();

    print!("{}[2J", 27 as char);
    //cool numbers
    let mut clock = [hour, min];

    while true {
        //println!("{} : {}", hour, min);

        match clock {
            [12, 0 ..= 29] => println!("               J=====L
            J===  12 ===L
           === 11 #  1 ===
          II      #      II
         II 10    #     2 II
        II        #        II
        II 9     <#>     3 II
        II                 II
         II 8           4 II
          II             II
           === 7     5 ===
            )===  6  ===(
               )=====("),
            [12, 30 ..= 60] => println!("               J=====L
            J===  12 ===L
           === 11   # 1 ===
          II       #     II
         II 10     #    2 II
        II        #        II
        II 9     <#>     3 II
        II                 II
         II 8           4 II
          II             II
           === 7     5 ===
            )===  6  ===(
               )=====("),
            [1, 0 ..= 29] => println!("               J=====L
            J===  12 ===L
           === 11    1 ===
          II        #    II
         II 10     #    2 II
        II         #       II
        II 9     <#>     3 II
        II                 II
         II 8           4 II
          II             II
           === 7     5 ===
            )===  6  ===(
               )=====("),
            [1, 30 ..= 60] => println!("               J=====L
            J===  12 ===L
           === 11    1 ===
          II         #   II
         II 10      #   2 II
        II         #       II
        II 9     <#>     3 II
        II                 II
         II 8           4 II
          II             II
           === 7     5 ===
            )===  6  ===(
               )=====("),
            [2, 0 ..= 29] => println!("               J=====L
            J===  12 ===L
           === 11    1 ===
          II             II
         II 10       ## 2 II
        II         ##      II
        II 9     <#>     3 II
        II                 II
         II 8           4 II
          II             II
           === 7     5 ===
            )===  6  ===(
               )=====("),
            [2, 30 ..= 60] => println!("               J=====L
            J===  12 ===L
           === 11    1 ===
          II             II
         II 10          2 II
        II         ######  II
        II 9     <#>     3 II
        II                 II
         II 8           4 II
          II             II
           === 7     5 ===
            )===  6  ===(
               )=====("),
            [3, 0 ..= 29] => println!("               J=====L
            J===  12 ===L
           === 11    1 ===
          II             II
         II 10          2 II
        II                 II
        II 9     <###### 3 II
        II                 II
         II 8           4 II
          II             II
           === 7     5 ===
            )===  6  ===(
               )=====("),
            [3, 30 ..= 60] => println!("               J=====L
            J===  12 ===L
           === 11    1 ===
          II             II
         II 10          2 II
        II                 II
        II 9     <#>     3 II
        II         ######  II
         II 8           4 II
          II             II
           === 7     5 ===
            )===  6  ===(
               )=====("),
            [4, 0 ..= 29] => println!("               J=====L
            J===  12 ===L
           === 11    1 ===
          II             II
         II 10          2 II
        II                 II
        II 9     <#>     3 II
        II         ##      II
         II 8        ###4 II
          II             II
           === 7     5 ===
            )===  6  ===(
               )=====("),
            [4, 30 ..= 60] => println!("               J=====L
            J===  12 ===L
           === 11    1 ===
          II             II
         II 10          2 II
        II                 II
        II 9     <#>     3 II
        II         #       II
         II 8       ##  4 II
          II          #  II
           === 7     5 ===
            )===  6  ===(
               )=====("),
            [5, 0 ..= 29] => println!("                J=====L
            J===  12 ===L
           === 11    1 ===
          II             II
         II 10          2 II
        II                 II
        II 9     <#>     3 II
        II         #       II
         II 8       #   4 II
          II         #   II
           === 7     5 ===
            )===  6  ===(
               )=====("),
            [5, 30 ..= 60] => println!("               J=====L
            J===  12 ===L
           === 11    1 ===
          II             II
         II 10          2 II
        II                 II
        II 9     <#>     3 II
        II        #        II
         II 8      #    4 II
          II       #     II
           === 7   # 5 ===
            )===  6  ===(
               )=====("),
            [6, 0 ..= 29] => println!("                J=====L
            J===  12 ===L
           === 11    1 ===
          II             II
         II 10          2 II
        II                 II
        II 9     <#>     3 II
        II        #        II
         II 8     #     4 II
          II      #      II
           === 7  #  5 ===
            )===  6  ===(
               )=====("),
            [6, 30 ..= 60] => println!("               J=====L
            J===  12 ===L
           === 11    1 ===
          II             II
         II 10          2 II
        II                 II
        II 9     <#>     3 II
        II        #        II
         II 8    #      4 II
          II     #       II
           === 7 #   5 ===
            )===  6  ===(
               )=====("),
            [7, 0 ..= 29] => println!("                J=====L
            J===  12 ===L
           === 11    1 ===
          II             II
         II 10          2 II
        II                 II
        II 9     <#>     3 II
        II        #        II
         II 8    #      4 II
          II    #        II
           === 7     5 ===
            )===  6  ===(
               )=====("),
            [7, 30 ..= 60] => println!("               J=====L
            J===  12 ===L
           === 11    1 ===
          II             II
         II 10          2 II
        II                 II
        II 9     <#>     3 II
        II        #        II
         II 8   ##      4 II
          II   #         II
           === 7     5 ===
            )===  6  ===(
               )=====("),
            [8, 0 ..= 29] => println!("                J=====L
            J===  12 ===L
           === 11    1 ===
          II             II
         II 10          2 II
        II                 II
        II 9     <#>     3 II
        II       #         II
         II 8  ##       4 II
          II             II
           === 7     5 ===
            )===  6  ===(
               )=====("),
            [8, 30 ..= 60] => println!("               J=====L
            J===  12 ===L
           === 11    1 ===
          II             II
         II 10          2 II
        II                 II
        II 9     <#>     3 II
        II    ####         II
         II 8           4 II
          II             II
           === 7     5 ===
            )===  6  ===(
               )=====("),
            [9, 0 ..= 29] => println!("                J=====L
            J===  12 ===L
           === 11    1 ===
          II             II
         II 10          2 II
        II                 II
        II 9 ######>     3 II
        II                 II
         II 8           4 II
          II             II
           === 7     5 ===
            )===  6  ===(
               )=====("),
            [9, 30 ..= 60] => println!("               J=====L
            J===  12 ===L
           === 11    1 ===
          II             II
         II 10          2 II
        II   ######        II
        II 9     <#>     3 II
        II                 II
         II 8           4 II
          II             II
           === 7     5 ===
            )===  6  ===(
               )=====("),
            [10, 0 ..= 29] => println!("               J=====L
            J===  12 ===L
           === 11    1 ===
          II             II
         II 10 ###      2 II
        II        #        II
        II 9     <#>     3 II
        II                 II
         II 8           4 II
          II             II
           === 7     5 ===
            )===  6  ===(
               )=====("),
            [10, 30 ..= 60] => println!("              J=====L
            J===  12 ===L
           === 11    1 ===
          II   ##        II
         II 10   #      2 II
        II        #        II
        II 9     <#>     3 II
        II                 II
         II 8           4 II
          II             II
           === 7     5 ===
            )===  6  ===(
               )=====("),
            [11, 0 ..= 29] => println!("               J=====L
            J===  12 ===L
           === 11    1 ===
          II     #       II
         II 10    #     2 II
        II        #        II
        II 9     <#>     3 II
        II                 II
         II 8           4 II
          II             II
           === 7     5 ===
            )===  6  ===(
               )=====("),
            [11, 30 ..= 60] => println!("              J=====L
            J===  12 ===L
           === 11#   1 ===
          II     #       II
         II 10    #     2 II
        II        #        II
        II 9     <#>     3 II
        II                 II
         II 8           4 II
          II             II
           === 7     5 ===
            )===  6  ===(
               )=====("),
            _ => println!("Error")
        };
        sleep(Duration::new(60, 0));

        print!("{}[2J", 27 as char);
        clock[1]+=1;
        if clock[1] == 60{
            clock[1] = 0;
            clock[0]+=1;
        } else if clock[0] == 23{
            clock[0] = 0;
        }
    }
}
