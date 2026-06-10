pub mod game;
use game::Player::Player;
use game::GameMap::GameMap;
use std::io;

fn main() {
    let map = GameMap::new();
    println!("| 1) Finland | 2) Sweden | 3) Norway | 4) Denmark |\nChoose your country: ");
    let mut chosen_country=String::new();
    io::stdin()
        .read_line(&mut chosen_country)
        .expect("Error reading input");
    let chosen_country:usize = chosen_country.trim().parse().expect("Error parsing!");
    let mut player:Player =match chosen_country{
        1 => Player::new(map.get_country_by_index(1).clone()),
        2 => Player::new(map.get_country_by_index(3).clone()),
        3 => Player::new(map.get_country_by_index(2).clone()),
        4 => Player::new(map.get_country_by_index(0).clone()),
        _ =>{
            println!("Invalid input!");
            return
        }
    };
    

    loop{
        let mut insp = String::new();
        println!("| Inspection on your own nation? | y = yes | n = no |");
        io::stdin()
            .read_line(&mut insp)
            .expect("Error reading input");
        insp = insp.trim().to_lowercase();
        if insp=="y"{
            println!("An inspection has been completed..");
            player.inspect();
        }else if insp =="n"{
            println!("The leader is confident. No inspection needed.");
        }
        let mut spy = String::new();
        println!("| 1) Spy on a country | 2) Invade a country | 0) Exit program |");
        io::stdin()
            .read_line(&mut spy)
            .expect("Error reading input");
        let spy:i32 = spy.trim().parse().expect("Error parsing!");
        if spy==1{
            map.list_countries();
            let mut idx = String::new();
            io::stdin()
                .read_line(&mut idx)
                .expect("Error reading input");
            let idx:usize = idx.trim().parse().expect("Error parsing to usize!");
            let spied_country = map.get_country_by_index(idx-1);
            if player.get_country() == *spied_country{
                println!("You can't spy on your own nation!");
            }else{
                println!("Espionage successful.\nCountry information:");
                println!("Name: {}", spied_country.get_name());
                println!("Population: {}", spied_country.get_population());
                println!("Army size: {}", spied_country.get_army_size());
            }
        }else if spy==2{
            map.list_countries();
            let mut idx = String::new();
            io::stdin()
                .read_line(&mut idx)
                .expect("Error reading input");
            let idx:usize = idx.trim().parse().expect("Error parsing to usize!");
            let target = map.get_country_by_index(idx-1);
            player.conquer_nation(target,target.get_name());
        }else if spy==0{
            break;
        }else{
            println!("Invalid game input. Try again.");
        }
        
    }
    
}
