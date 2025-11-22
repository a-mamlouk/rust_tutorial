fn main() {
    println!("Hello, world!\n");
    let x = 5;
    print!("The value of x is: {}\n", x);
    {
        let x = 2;
        print!("The value of x is: {}\n", x);
    }
    let x = x + 1;
    print!("The value of x is: {}\n", x);
    let x = "Shit this is working cause i redined x";
    print!("The value of x is: {}\n", x);
    const SECONDS_IN_MINUTES: u32 = 60;
    print!("The value of SIM is: {}\n", SECONDS_IN_MINUTES);
    //SECONDS_IN_MINUTES = 9; doesnt work cause it's a constant duhhh ...

    let number: u8 = 10;
    print!("the value of the unsigned number is {}.\n", number);

    let floating_point: f32 = 10.9;
    print!("the value of the floating number is {}.\n", floating_point);

    let booly: bool = true;
    print!("the value of booly is {}.\n", booly);

    let letter: char = 'A';
    print!("la valeur de letter est : {}\n", letter);

    //Tuples
    let tuple = (1, true, 's');
    let mut tuple2: (i8, bool, char) = (1, true, 's');
    print!("my 1st element in tuple {}\n", tuple.0);
    print!("my 3rd element in tuple2 {}\n", tuple2.2);
    tuple2.2 = 'x';
    print!("my new 3rd element in tuple2 {}\n", tuple2.2);

    //Arrays
    let mut array: [i8; 5] = [1, 2, 3, 4, 5];
    print!("my 4th element in array is: {}\n", array[3]);
    array[4] = 10;
    print!("my new 4th element in array is: {}\n", array[4]);   
}

//cargo init
//cargo build
//cargo run
//rustfmt main.rs

// This is a simple Rust program that prints "Hello, world!" to the console.
// commands used to run this program:
// 1. Save the code in a file named `main.rs` inside a folder named
// 2. Open a terminal and navigate to the folder containing `main.rs`.
// 3. Run the command `rustc main.rs` to compile the program.
// 4. After compilation, an executable file named `main` (or `main.exe` on Windows) will be created in the same folder.
// 5. Run the executable by typing `./main` (or `main.exe` on Windows) in the terminal.
// 6. You should see the output: `Hello, world!` `src`.
//// Note: Make sure you have Rust installed on your system to compile and run the program.
