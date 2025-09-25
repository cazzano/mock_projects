package main

import "fmt"

func main(){

fmt.Println("Let's start now")

// Just variables haah

villan:="luca"
gangster:="shelby"
choice:="none"
fmt.Println("Enter Your Choice -->")
fmt.Scanln(&choice)
if choice =="villan"{

fmt.Println(villan)

} else if choice=="gangster" {

fmt.Println(gangster)

} else {

fmt.Println("Invalid choice you bastaard!!")

}


// Now it's time for constants haah


const linux="arch"

fmt.Printf("Aaahhh You can't change the Arch btw cuz, I use Arch btw %s\n",linux)


// Now it's time for switch haah


sigma:="peaky_blinders"
omega:="vincenzo_cazzano"
fucker:="agent_47"
fmt.Println("who's Your Best? Tell Me")
fmt.Scanln(&choice)
switch choice{

case "sigma":
        fmt.Println(sigma)
case "omega":
        fmt.Println(omega)
case "fucker":
        fmt.Println(fucker)        
default:
    fmt.Println("Wrong Move Haah")
}

// Now the let's try loops haah

for i:=1;i<=5;i++{
fmt.Println("Fuck You!!!")
}

i:=1
for i<=5{
fmt.Println("OKay Captian Price I am In My Way!!!")
i++
say("Lan te char")
}

}


// This is our function haah


func say(word string){
fmt.Println("That's it haah",word)
}