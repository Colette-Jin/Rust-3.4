use super::Country::Country;

pub struct GameMap{
    countries:Vec<Country>,
}
impl GameMap{
    pub fn new()->Self{
        Self{
            countries:vec![
                Country::new(String::from("Denmark"), 6000000, 50000, vec![], false),
                Country::new(String::from("Finland"), 5600000, 900000, vec![], false),
                Country::new(String::from("Norway"), 5500000, 100000, vec![], false),
                Country::new(String::from("Sweden"), 10000000, 200000, vec![], false),
                
            ],
            
        }
    }//create all four countries with their data as Country type 
    // and store them in the country attribute of the GameMap struct

    pub fn list_countries(&self){
        let mut idx = 0;
        for country in &self.countries{
            println!("{}) {}",idx+1,country.get_name());
            idx +=1;
        }
    }//lists all countries and their indices:
    // "1) Finland" etc.

    pub fn get_country_by_index(&self,idx:usize)->&Country{
        &self.countries[idx]
    }//the given country in the GameMap attribute's countries
}