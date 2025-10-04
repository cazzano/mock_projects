use text_io::scan;

 pub fn traid_rank(){
    print!("\nHere We Go To The Fight Haah\n\n");
    loop{
        print!("\nSelect Your Fighting Method To Show Your Stats\n\n1.Attacked With Fists  2.Attacked With Arms  3.Attacked With Arms Inside Vehicle\n\n--> ");
        let method:i32;
        scan!("{}",method);
        if method==1{
          loop {
            print!("\nHow much you damage Your Opponents\n\n1.Normal Combos  2.WingChun  3. Multiple Martial Arts\n\n--> ");
            let fists:i32;
            scan!("{}",fists);
            if fists==1{
                print!("\nStates ==>  1.Traid --> 20-30%  2.Xp --> 10-15%  3.Level --> 0.2-0.5\n\n");
            } else if fists==2{
                print!("\nStates ==>  1.Traid --> 30-40%  2.Xp --> 15-20%  3.Level --> 0.5-0.7\n\n");
            } else if fists==3{
                print!("\nStates ==>  1.Traid --> 40-50%  2.Xp --> 20-25%  3.Level --> 0.7-1.0\n\n");
            } else {break}
          }
        } else if method==2{
          loop{
            print!("\nHow Much you damage Your Opponents\n\n1.Normal Gun With Cover  2.Big Guns With Cover Follow Ups  3.Every Gun With Massive Chaos\n\n--> ");
            let arms:i32;
            scan!("{}",arms);
            if arms==1{
                print!("\nStates ==>  1.Traid --> 25-35%  2.Xp --> 15-25%  3.Level --> 0.5-0.7\n\n");
            }else if arms==2{
                print!("\nStates ==>  1.Traid --> 35-45%  2.Xp --> 25-35%  3.Level --> 0.7-1.0\n\n");
            }else if arms==3{
                print!("\nStates ==>  1.Traid --> 45-55%  2.Xp --> 35-45%  3.Level --> 1.0-1.2\n\n");
            } else{break}
          }
        } else if method==3{
          loop{
            print!("\nHow Much You Damage Your Opponents\n\n1.Killed Enemies By Bike  2.Killed Enemies By Cars  3.Killed Enemies By Bike And Car, Massively\n\n--> ");
            let vehicle:i32;
            scan!("{}",vehicle);
            if vehicle==1{
                print!("\nStates ==>  1.Traid --> 55-60%  2.Xp --> 25-30%  3.Level --> 0.7-1.0\n\n");
            } else if vehicle==2{
                print!("\nStates ==>  1.Traid --> 60-65%  2.Xp --> 30-35%  3.Level --> 1.0-1.2\n\n");
            } else if vehicle==3{
                print!("\nStates ==>  1.Traid --> 65-70%  2.Xp --> 35-40%  3.Level --> 1.2-1.5\n\n");
            } else {break}
          }
        } else {break}

    }
  }
