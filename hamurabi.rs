//Hamurabi
/** This requires the following Cargo.tomlL
-------------
[package]
name = "hamurabi"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8.5"

[[bin]]
name = "hamurabi"
path = "src/hamurabi.rs"
-------------
*/

use rand::Rng;

const OGH: &str = "O Great Hammurabi!";

struct Sumeria {
	year: i32,
	population: i32,
	grain: i32,
	acres: i32,
	land_value: i32,
	starved: i32,
	percent_starved: i32,
	plague_victims: i32, 
	immigrants: i32, 
	grain_harvested: i32, 
	harvest_per_acre: i32,
	amount_eaten_by_rats: i32,
	grain_fed_to_people: i32,
	acres_planted: i32,
	still_in_office: bool,
}

//--------------------------------

//utility functions
//get random number from 0 .. n-1
fn get_random(n: i32) -> i32 {
	let mut rng = rand::thread_rng();
	let r: f64 = rng.gen::<f64>();
	let f2: f64 = r * (n as f64);
	let a: i32 = f2 as i32;	
	return a;
}

//get number from user
fn get_number() -> i32 {
	let mut buff = String::new();
	let stdin = std::io::stdin(); 
	let _ = stdin.read_line(&mut buff);
	let buff = buff.trim();
	let n: i32 = buff.parse::<i32>().expect("Not a valid number");
	return n;
}

fn jest() {
	println!("{}, surely you jest!", OGH);
}

//starts with an homage to the Basic source code
fn print_intro() {
	let intro = r#"	
HAMURABI 
CREATIVE COMPUTING  MORRISTOWN, NEW JERSEY.

Congratulations! You are the newest ruler of ancient Samaria,
elected for a ten year term of office. Your duties are to
dispense food, direct farming, and buy and sell land as
needed to support your people. Watch out for rat infestations
and the plague! Grain is the general currency, measured in
bushels.

The following will help you in your decisions:                
   * Each person needs at least 20 bushels of grain per year to survive
   * Each person can farm at most 10 acres of land
   * It takes 2 bushels of grain to farm an acre of land
   * The market price for land fluctuates yearly
     
Rule wisely and you will be showered with appreciation at the
end of your term. Rule poorly and you will be kicked out of office!

(Hint: You should feed 20 bushels of grain per person 
and plant 2 bushel per acre. Each person can farm 10 acres)
	"#;
	println!("{}", intro);
}

//----------------------------------------------

//Sumeria class
impl Sumeria {
    fn new() -> Self {
   	Self{
    		year: 0,
    	    	population: 100,
    		grain: 3800,		//was 2800;
    		acres: 1000,
    		land_value: 19,
    		starved: 0,
    		percent_starved: 0,
    		plague_victims: 0,
    		immigrants: 5,
    		grain_harvested: 4000,
    		harvest_per_acre: 4,
    		amount_eaten_by_rats: 200,
    		grain_fed_to_people: 2000,
    		acres_planted: 1000,
    		still_in_office: true,
    	}
    }
    
    /**
     * Prints the year-end summary.
     */
	fn print_summary(&self) {
	        println!("___________________________________________________________________");
	        println!("{}",OGH);
	        println!("You are in year {} of your ten year rule.", self.year);
	        if self.plague_victims > 0 {
	            println!("A horrible plague killed {} people.", self.plague_victims);
	        }
	        println!("In the previous year {} people starved to death,",self.starved);
	        println!("and {} people entered the kingdom.",self.immigrants);
	        println!("The population is now {}.",self.population);
	        println!("We harvested {} bushels at {} bushels per acre.",self.grain_harvested,self.harvest_per_acre);
	        if self.amount_eaten_by_rats > 0 {
	            println!("*** Rats destroyed {} bushels, leaving {} bushels in storage.",self.amount_eaten_by_rats,self.grain);
	        } else {
	            println!("We have {} bushels of grain in storage.", self.grain);
	        }
	        println!("The city owns {} acres of land.", self.acres);
	        println!("Land is currently worth {} bushels per acre.", self.land_value);
	        println!();
    } 
    
    /**
     * Allows the user to buy land.
     */
    fn buy_land(&mut self) {
        let q: &str = "How many acres of land will you buy? ";
        println!("{}",q);
        let mut acres_to_buy: i32 = get_number();
        let mut cost: i32 = self.land_value * acres_to_buy;
        while cost > self.grain {
        	jest();
            println!("We have but {} bushels of grain, not {} !",self.grain,cost);
            println!("{}",q);
            acres_to_buy = get_number();
            cost = self.land_value * acres_to_buy;
        }
        self.grain = self.grain - cost;
        self.acres = self.acres + acres_to_buy;
        println!("{}, you now have {} acres of land",OGH,self.acres);
        println!("and {} bushels of grain.",self.grain);
    }

    /**
     * Allows the user to sell land.
     */
    fn sell_land(&mut self) {
    	let q: &str = "How many acres of land will you sell? ";
    	println!("{}",q);
    	let mut acres_to_sell: i32 = get_number();

        while acres_to_sell > self.acres {
            jest();
            println!("We have but {} acres!",self.acres);
            println!("{}",q);
            acres_to_sell = get_number();
        }
        self.grain = self.grain + self.land_value * acres_to_sell;
        self.acres = self.acres - acres_to_sell;
        println!("{}, you now have {} acres of land",OGH,self.acres);
        println!("and {} bushels of grain.",self.grain);
    }    

    /**
     * Allows the user to decide how much grain to use to feed people.
     */
    fn feed_people(&mut self) {
        let q: &str = "How much grain will you feed to the people? ";
        println!("{}",q);
        self.grain_fed_to_people = get_number();

	while self.grain_fed_to_people > self.grain {
            jest();
            println!("We have but {} bushels!",self.grain);
            println!("{}",q);
        	self.grain_fed_to_people = get_number();
        }
        self.grain = self.grain - self.grain_fed_to_people;
        println!("{}, {} bushels of grain remain.",OGH,self.grain);
    }

    /**
     * Allows the user to choose how much grain to plant.
     */
    fn plant_grain(&mut self) {
        let q: &str = "How many bushels will you plant? ";
        let mut amount_to_plant: i32 = 0;
        let mut have_good_answer: bool = false;

        while !have_good_answer {
        	println!("{}",q);
            	amount_to_plant = get_number();
            	if amount_to_plant > self.grain {
                	jest();
                	println!("We have but {} bushels left!",self.grain);
            	} else if amount_to_plant > 2 * self.acres {
                jest();
                println!("We have but {} acres available for planting!",self.acres);
            } else if amount_to_plant > 20 * self.population {
                jest();
                println!("We have but {} people to do the planting!",self.population);
            } else {
                have_good_answer = true;
            }
        }
        self.acres_planted = amount_to_plant / 2;
        self.grain = self.grain - amount_to_plant;
        println!("{}, we now have {} bushels of grain in storage.",OGH,self.grain);
    }

    /**
     * Checks for plague, and counts the victims.
     */
    fn check_for_plague(&mut self) {
        let chance: i32 = get_random(100);
        if chance < 15 {
        	println!("*** A horrible plague kills half your people! ***");
            	self.plague_victims = (self.population / 2) as i32;
            	self.population = self.population - self.plague_victims;
        } else {
            self.plague_victims = 0;
        }
    }

    /**
     * Counts how many people starved, and removes them from the population.
     */
    fn count_starved_people(&mut self) {
        let people_fed: i32 = (self.grain_fed_to_people / 20) as i32;
        if people_fed >= self.population {
            self.starved = 0;
            self.percent_starved = 0;
            println!("Your people are well fed and happy.");
        } else {
            self.starved = self.population - people_fed;
            println!("{} people starved to death.",self.starved);
            self.percent_starved = ((100 * self.starved) / self.population) as i32;
            self.population = self.population - self.starved;
        }
    }

    /**
     * Counts how many people immigrated.
     */
   fn count_immigrants(&mut self) {
        if self.starved > 0 {
            self.immigrants = 0;
        } else {
            self.immigrants = (20 * self.acres + self.grain) / (100 * self.population) + 1;
            self.population += self.immigrants;
        }
    }

    /**
     * Determines the harvest, and collects the new grain.
     */
    fn take_in_harvest(&mut self) {
        self.harvest_per_acre = get_random(6) + 1;
        self.grain_harvested = self.harvest_per_acre * self.acres_planted;
        self.grain = self.grain + self.grain_harvested;
    }

    /**
     * Checks if rats get into the grain, and determines how much they eat.
     */
    fn check_for_rats(&mut self) {
        if get_random(100) < 40 {
            let percent_eaten_by_rats: i32 = 10 + get_random(21);
            println!("*** Rats eat {} percent of your grain! ***",percent_eaten_by_rats);
            self.amount_eaten_by_rats = ((percent_eaten_by_rats * self.grain) / 100) as i32;
            self.grain = self.grain - self.amount_eaten_by_rats;
        } else {
            self.amount_eaten_by_rats = 0;
        }
    }

    /**
     * Randomly sets the new price of land.
     */
    fn update_land_value(&mut self) {
        self.land_value = 17 + get_random(9);	//this was 7
    }

    /**
     * Prints an evaluation at the end of a game.
     */
    fn print_final_score(&self) {
	let mut score:i32 = self.acres;
     	if 20 * self.population < score {
     		score = 20 * self.population
     	}

        if self.starved >= (45 * self.population) / 100 {
        	println!("O Once-Great Hammurabi");
		println!("{} of your people starved during the last year of your",self.starved);
		println!("incompetent reign! The few who remain have stormed the palace");
            println!("and bodily evicted you!");
            println!("\nYour final rating: TERRIBLE.");
            return;
        }

        if score < 600 {
            println!("Congratulations, {}!",OGH);
            println!("You have ruled wisely but not");
            println!("well; you have led your people through ten difficult years, but");
            println!("your kingdom has shrunk to a mere {} acres.",self.acres);
            println!("Your final rating: ADEQUATE.");
        } else if score < 800 {
            println!("Congratulations, {}! You  have ruled wisely, and",OGH);
            println!("shown the ancient world that a stable economy is possible.");
            println!("\nYour final rating: GOOD.");
        } else {
            println!("Congratulations, {} You  have ruled wisely and well, and",OGH);
            println!("expanded your holdings while keeping your people happy.");
            println!("Altogether, a most impressive job!");
            println!("\nYour final rating: SUPERB.");
        }
    }
}

//===============================

fn play_game() {
    	let mut g = Sumeria::new();
	g.still_in_office = true;
        g.print_summary();

	for y in 1..10 {
		if !g.still_in_office {
			break;
		}
        	g.year = y;
        	println!("Kingdom of Sumeria, Year: {}",g.year);	
            	g.buy_land();
            	g.sell_land();
            	g.feed_people();
            	g.plant_grain();
            	g.check_for_plague();
            	g.count_starved_people();
            	if g.percent_starved >= 45 {
            	    g.still_in_office = false;
            	}
            	g.count_immigrants();
            	g.take_in_harvest();
            	g.check_for_rats();
            	g.update_land_value();
            	g.print_summary();
        }
        g.print_final_score();
}

fn main() {
	print_intro();
	play_game();
}

