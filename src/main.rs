use text_io::scan;

fn main() {
    print!("\nHello, world!\n\n");
    let gangster = "shelby";
    let lora=6;
    loop{
        print!("\nEnter Your Choice Haah\n\n1.Gangster  2.Lora\n\n--> ");
        let choice: i64;
        scan!("{}",choice);
        if choice==1{
            print!("{}",gangster);
        } else if choice==2{
            print!("{}",lora);
        } else { break }
    }

}
