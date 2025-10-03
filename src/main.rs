use text_io::scan;

fn main() {
    print!("\nHello, world!\n\n");
    let gangster = "shelby";
    let lora=6;
    loop{
        print!("\nEnter Your Choice Haah\n\n1.Gangster  2.Lora  3.Run The Function\n\n--> ");
        let choice: i64;
        scan!("{}",choice);
        if choice==1{
            print!("{}",gangster);
        } else if choice==2{
            print!("{}",lora);
        } else if choice ==3{
            show_values("Hi I Am Shelby","And My Company Is An Private Limited");
        } else { break }
    }

}

fn show_values(a: &str,b: &str){
    print!("\n{}\n\n{}\n",a,b);
}
