package gta_projects 

import "fmt"


func Crimes_func(){

fmt.Printf("\nNow We Are Going To Do Some Crime\n\n")


for {
fmt.Printf("\nChoose Your Crime Haah\n\n1.Shoot Someone  2.Punch Someone  3.Destroy City\n\n--> ")
var crime int64
fmt.Scanf("%d",&crime)
if crime==1{

for {
fmt.Printf("\nHow much you shoot ?\n\n1.Killed Civilian  2.Killed Police Officer  3.Killed Multiple Civilians\n\n--> ")
var shoot int64
fmt.Scanf("%d",&shoot)
if shoot==1{
fmt.Printf("\nYou Got These Things Bro\n\n1.Stars -- **  2.Police is chasing you\n\n")
} else if shoot==2{

fmt.Printf("\nYou Got These Things Bro\n\n1.Stars -- ***  2.Police Cars Don't Care And Wanted To Fuck You\n\n")

} else if shoot==3 {

fmt.Printf("\nYou Got These Things Bro\n\n1.Stars -- ***  2.Police Still Wants To Fuck You With Lust\n\n")
} else { break }


   } } else if crime ==2{

	 for {  
         var punch int64
	 fmt.Printf("\nTo Who, You Punch?\n\n1.Civilian  2.Police Officer\n\n--> ")
	 fmt.Scanf("%d",&punch)
	 if punch==1{
                fmt.Printf("\nYou Got These Things Brother\n\n1.Stars -- *  2.You Can Easily Escape From Police Onfoot\n\n")

	 } else if punch ==2{

               fmt.Printf("\nYou Got These Things Brother\n\n1.Stars -- **  2.Police Wants To Take Revenge From You\n\n")
	 } else { break }
 }

   } else if crime ==3 {

          for {

	       fmt.Printf("\nWhat Type Of Destruction You Made\n\n1.Destroys Everything Insanely  2.Destroys In Limits\n\n-->  ")
	       var destroy int64
	       fmt.Scanf("%d",&destroy)
	       if destroy==1 {
                   fmt.Printf("\nYou Got These Things Bro\n\n1.Stars -- *****  2.Police Wants You Dead\n\n")
                   
	       } else if destroy ==2{

                    fmt.Printf("\nYou Got These Things Bro\n\n1.Stars -- ****  2.Police Wants You EveryWhere\n\n")
	       } else { break }


	  }


   } else { break}

}}
