#[derive(PartialEq, Clone)]
pub struct Country{
    name:String,
    population: i64,
    army_size: i64,
    conquered_countries:Vec<String>,
    is_conquered:bool,
}
impl Country{
    pub fn new(name:String,population:i64,army_size:i64,conquered_countries:Vec<String>,is_conquered:bool)->Self{
        Self{
            name,
            population,
            army_size,
            conquered_countries,
            is_conquered,
        }
    }
    pub fn get_name(&self)-> &str{
        &self.name
    }
    pub fn get_population(&self)->&i64{
        &self.population
    }
    pub fn get_army_size(&self)->&i64{
        &self.army_size
    }
    pub fn get_conquered_nations(&self)->&Vec<String>{
        &self.conquered_countries
    }
    pub fn get_is_conquered(&self)->&bool{
        &self.is_conquered
    }
    pub fn set_population(&mut self,size:i64){
        self.population +=size;
    }
    pub fn set_army_size(&mut self,size:i64){
        self.army_size += size;
    }
    pub fn set_conquered_nations(&mut self,country:String){
        self.conquered_countries.push(country);
    }
    pub fn set_is_conquered(&mut self,counquered:bool){
        self.is_conquered = counquered;
    }
}