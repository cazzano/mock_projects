package gta_v

import "fmt"
import "strconv"
import "strings"

// Function to reveal crew member cuts
func revealCuts(crew_members []string, crew_cuts map[string]string) {
    fmt.Printf("\nEnter Your Crew Member To Reveal His Cut\n\n--> ")
    for{
        var reveal string
        fmt.Printf("\nThe Crew Members Were\n\n%s\n\n--> ",crew_members)
        
        fmt.Scanf("%s",&reveal)
        
        if reveal=="stop"{ break }
        fmt.Printf("\nThe Cut Was %s\n\n--> ",crew_cuts[reveal])
    }
}

// Function to calculate crew member cuts
func calculateCuts(crew_members []string, crew_cuts map[string]string, total_money float64) {
    fmt.Printf("\nSelect Your Crew Member's Cut\n\n--> ")
    
    for{
        var selected_crew string
        fmt.Printf("\nThe Crew Members Were\n\n%s\n\n--> ",crew_members)
        fmt.Scanf("%s",&selected_crew)
        if selected_crew=="stop"{ break }
        converted_cut:=crew_cuts[selected_crew]
        converted_cut = strings.TrimSuffix(converted_cut, "%")
        member_cut, _ := strconv.ParseFloat(converted_cut, 64)
        var calculated_cut float64
        calculated_cut=total_money*(member_cut/100)
        fmt.Printf("\nThe Calculated Cut Is %f\n\n",calculated_cut)
    }
}

func Cuts_func(){

    total_money:=56245454522.55454

    fmt.Printf("\nHere We Go To Rob A Bank\n\nAnd We Got %f\n\n",total_money)

    crew_members:=make([]string,0)
    loop_manager:="none"

    fmt.Printf("\n--> !!! Be Advised Use the ```stop``` key in order to stop entering okay !!!\n\nNow Enter Crew Members\n\n--> ")

    for{
        fmt.Scanln(&loop_manager)
        if loop_manager=="stop"{break}
        crew_members=append(crew_members,loop_manager)
    }

    // Now map crew members to their cuts
    crew_cuts := make(map[string]string)
        
    fmt.Printf("\nNow Enter Cuts for Each Crew Member (type 'stop' to finish)\n\n--> ")

    for {
        fmt.Printf("\nNow Select Your Crew Member\n\n%s\n\n--> ",crew_members)     
        var member_name, cut string
        
        fmt.Scanf("%s",&member_name)
        
        if member_name == "stop" { 
            break 
        }
        
        fmt.Printf("\nEnter Cut for %s\n\n--> ", member_name)
        fmt.Scanf("%s",&cut)
        
        if cut == "stop" { 
            break 
        }
        
        crew_cuts[member_name] = cut
    }
    for {
    var choice int64
    fmt.Printf("\nEnter Your Choice And You Have Two Choices\n\n1.Reveal Crew's Cut\n2.Calculate Crew's Cut\n\n--> ")
    fmt.Scanf("%d",&choice)
    if choice==1{ revealCuts(crew_members, crew_cuts)

    } else if choice==2 {
     
    calculateCuts(crew_members, crew_cuts, total_money)
    

    } else { break  }


    

    // Call the functions
  //  revealCuts(crew_members, crew_cuts)
  //  calculateCuts(crew_members, crew_cuts, total_money)
    }
}
