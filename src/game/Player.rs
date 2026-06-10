use super::Country::Country;

pub struct Player{
    country: Country,
}
impl Player{
    pub fn new(country:Country)->Self{
        Self{
            country
        }
    }//instantiates the Player object with the given country
    pub fn inspect(&self) {
        println!("Country information:");
        println!("Name: {}", self.country.get_name());
        println!("Population: {}", self.country.get_population());
        println!("Army size: {}", self.country.get_army_size());
    }
    pub fn get_country(&self)->Country{
        self.country.clone()
    }//The `get_country` function returns a reference to the country selected by the player.
    pub fn conquer_nation(&mut self,target:&Country,name:&str){
        let mut target = target.clone();
        if *target.get_is_conquered(){
            println!("Fail! The target is already down!");
        }else if self.country==target{
            println!("Fail! You cannot invade yourself!");
        }else{
            if *self.country.get_army_size()>*target.get_army_size(){
                let _= self.country.set_conquered_nations(name.to_string());
                target.set_is_conquered(true);
                let _= self.country.set_population(*target.get_population());
                let _= self.country.set_army_size(*target.get_army_size());
            }else{
                println!("You have lost your war against {}. You have been conquered.",name);
            }
        }
    }//Check if the land is already conquered. 
    // Check if your destination country is your own country.

}