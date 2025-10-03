use text_io::scan;

fn main() {
    
    // Our Brand Function print!() Which Could Be Related To The fmt.Printf() Function by Go.

    print!("\nHello, world!\n\n");

    // And We Can Use Let To Add Variables Haah, Relatable To The var by Go.

    let gangster = "shelby";
    let lora=6;

    // We Can Use The loop flow, Relateable To The for by Go.

    loop{
        print!("\nEnter Your Choice Haah\n\n1.Gangster  2.Lora  3.Run The Function\n\n--> ");
        let choice: i64;
        
        // We Can Use The scan!(), Relatable To The fmt.Scanf() By Go.
        
        scan!("{}",choice);
         
        // The Flow Remains The Same As Go.

        if choice==1{
            print!("{}",gangster);
        } else if choice==2{
            print!("{}",lora);
        } else if choice ==3{
            show_values("Hi I Am Shelby","And My Company Is An Private Limited");
        } else { break }
    }

}

// We Can Make Functions By fn function(), Relatable To The func Functions() By Go.

fn show_values(a: &str,b: &str){
    print!("{}\n\n{}\n",a,b);
}
