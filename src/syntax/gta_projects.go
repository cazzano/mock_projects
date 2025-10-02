package main

import "fmt"
import "projects/gta_v"


func main(){

for {
var project int64
fmt.Printf("\nNow Select Project To Start\n\n1.Bank Hiest Cuts Calculator  2.Crime Responses\n-->  ")

fmt.Scanf("%d",&project)
if project==1{

gta_projects.Cuts_func()

} else if project==2 {

   gta_projects.Crimes_func()

} else { break }

}
}
