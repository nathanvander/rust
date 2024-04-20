//HuntTheWumpus
/** Cargo.toml
-------------
[package]
name = "hunt_the_wumpus"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8.5"

[[bin]]
name = "hunt_the_wumpus"
path = "src/hunt_the_wumpus.rs"
-------------
*/

use rand::Rng;

const ROOMS : [[usize; 3]; 20] = [[1, 4, 7],   [0, 2, 9],   [1, 3, 11],   [2, 4, 13], [0, 3, 5],
		[4, 6, 14],  [5, 7, 16],   [0, 6, 8],   [7, 9, 17],   [1, 8, 10],
		[9, 11, 18], [2, 10, 12], [11, 13, 19],  [3, 12, 14],  [5, 13, 15],
  		[14, 16, 19], [6, 15, 17],  [8, 16, 18], [10, 17, 19], [12, 15, 18] ]; 


fn is_room_adjacent(room_a: i32, room_b: i32) -> bool {
	for j in 0..3 {
		if ROOMS[room_a as usize][j as usize] == room_b as usize {
			return true;
		}
	}
	return false
}

// roomA must be 0..19, index must be 0..2
fn get_adjacent_room(room_a: i32, index: i32) -> i32 {
	return ROOMS[room_a as usize][index as usize] as i32;
}

fn get_random(n: i32) -> i32 {
	let mut rng = rand::thread_rng();
	let r: f64 = rng.gen::<f64>();
	let f2: f64 = r * (n as f64);
	let a: i32 = f2 as i32;	
	return a;
}

//get number from user
//-1 is invalid
fn input_number(q: &str) -> i32 {
	println!("{}",q);
	let mut buff = String::new();
	let stdin = std::io::stdin(); 
	let _ = stdin.read_line(&mut buff);

	//we need to handle the parse error	
	match buff.trim().parse::<i32>() {
        Ok(n) => return n,
        Err(_e) => return -1,
    }	
}

fn print_instructions() {
    println!(" Welcome to 'Hunt the Wumpus'! ");
    println!(" The wumpus lives in a cave of 20 rooms. Each room has 3 tunnels leading to");
    println!(" other rooms. (Look at a dodecahedron to see how this works - if you don't know");
    println!(" what a dodecahedron is, ask someone).");
    println!(" ");
    println!(" Hazards:");
    println!(" Bottomless pits - two rooms have bottomless pits in them. If you go there, you ");
    println!(" fall into the pit (& lose!)");
    println!(" Super bats - two other rooms have super bats.  If you go there, a bat grabs you");
	println!(" and takes you to some other room at random. (Which may be troublesome). Once the");
    println!(" bat has moved you, that bat moves to another random location on the map.");
	println!(" ");
    println!(" Wumpus:");
    println!(" The wumpus is not bothered by hazards (he has sucker feet and is too big for a");
    println!(" bat to lift).  Usually he is asleep.  Two things wake him up: you shooting an");
    println!(" arrow or you entering his room. If the wumpus wakes he moves one room or ");
    println!(" stays still. After that, if he is where you are, he eats you up and you lose!");
	println!(" ");
    println!(" You:");
    println!(" Each turn you may move, save or shoot an arrow using the commands move, save, & shoot.");
    println!(" Moving: you can move one room (thru one tunnel).");
    println!(" Arrows: you have 3 arrows. You lose when you run out. You aim by telling the");
    println!(" computer the rooms you want the arrow to go to.  If the arrow can't go that way");
    println!(" (if no tunnel), the arrow will not fire.");
	println!(" ");
    println!(" Warnings:");
    println!(" When you are one room away from a wumpus or hazard, the computer says:");
	println!(" Warnings:");
    println!(" Wumpus: 'I smell a wumpus'");
    println!(" Bat: 'Bats nearby'");
    println!(" Pit: 'I feel a draft'");
	println!(" Warnings:");
    println!(" ");
    println!("Press 1 to return to the main menu.");
    let _i: i32 = input_number(">");
}


//=================================================
struct WumpusGame {
	num_rooms: i32,
    current_room: i32,  
    starting_position: i32, 
    wumpus_room: i32, 
    bat1_room: i32,
    bat2_room: i32,
    pit1_room: i32, 
    pit2_room: i32, 
    wumpus_start: i32, 
    bat1_start: i32,
    bat2_start: i32,
    player_alive: bool, 
    wumpus_alive: bool, 
    num_arrows: i32,
}

impl WumpusGame {
    fn new() -> Self {
   	Self{
	num_rooms: 20,
    	current_room: -1,  
    	starting_position: -1, 
    	wumpus_room: -1, 
    	bat1_room: -1,
    	bat2_room: -1,
    	pit1_room: -1, 
    	pit2_room: -1, 
    	wumpus_start: -1, 
    	bat1_start: -1,
    	bat2_start: -1,
    	player_alive: true, 
    	wumpus_alive: true, 
    	num_arrows: -1,
    	}
    }

	fn place_wumpus(&mut self) {
		let random_room: i32 = get_random(19) + 1;
		self.wumpus_room = random_room;
		self.wumpus_start = random_room;
	}

	fn place_bats(&mut self) {
	  	let mut valid_room : bool= false;
	  	while !valid_room {
	    	self.bat1_room = get_random(19) + 1;
	      	if self.bat1_room != self.wumpus_room {
	         	valid_room = true;
	       	}
	  	}

	  	valid_room = false;
	  	while !valid_room {
	  		self.bat2_room = get_random(19) + 1;
	  		if self.bat2_room != self.wumpus_room && self.bat2_room != self.bat1_room {
	  			valid_room = true;
	  		}
	  	}
	  	self.bat1_start = self.bat1_room;
	  	self.bat2_start = self.bat2_room;
	}

	fn place_pits(&mut self) {
		self.pit1_room = get_random(19) + 1;
	   	self.pit2_room = get_random(19) + 1;
	}

	fn place_player(&mut self) {
		self.starting_position = 0;
		self.current_room = self.move_player(0);
	}

	//move player to new room and return room number
	//I don't know why this is a separate method, I am just translating the code
	//it is called on startup, when player moves, and when bats move him
	fn move_player(&mut self,r: i32) -> i32 {
		println!("[move_player] Player is moving from {} to room {}",self.current_room,r);
		return r;
	}

	fn action_move(&mut self) {
	   	let new_room:i32 = input_number("Which room? ");
	    if new_room < 0 || new_room > 19 {
	       	println!("You cannot move there.");
	    } else {
	    	// Check if the user inputted a valid room id, then simply move player there.
	        if self.is_valid_move(new_room)
	        {
	        	self.current_room = self.move_player(new_room);
	        	self.inspect_current_room();
			}
	        else
	        {
	         println!("There are no tunnels that lead there");
	        }
		}
	}

	fn is_valid_move(&self, roomid: i32) -> bool {
		if roomid < 0 {return false; }
		if roomid > self.num_rooms {return false;}
		if !is_room_adjacent(self.current_room, roomid) {return false;}
		return true;
	}

	fn shoot(&mut self) {
		if self.num_arrows < 1 {
	    	println!("You do not have any arrows!");
	        return;
		}

		println!("Which room? ");
		let new_room: i32 = input_number("?");
	    if new_room < 0 || new_room > 19 {
	    	println!("You cannot shoot there.");
	       	return;
		}        	

		if self.is_valid_move(new_room) {
	    	println!("You shoot an arrow from {} to {}", self.current_room, new_room);
	        //self.num_arrows--;	//Rust doesn't have postfix operators
	        self.num_arrows = self.num_arrows - 1;
	        if new_room == self.wumpus_room {
	        	println!("ARGH.. Splat!");
	            println!("Congratulations! You killed the Wumpus! You Win.");
	            println!("Awesome job dude!!!");
	            println!("Please don't ever play this stupid game again");
	            println!("Press 1 to return to the main menu.");
	            self.wumpus_alive = false;
	            input_number("?");
			}
	        else
	        {
	        	let rando: i32 = get_random(2);
	          	//there is 50% chance Wumpus will move
	           	if rando == 1 {
	            	println!("You miss! But you startled the Wumpus");
	                self.move_startled_wumpus(self.wumpus_room);
	                println!("Arrows Left: {}", self.num_arrows);
	                if self.wumpus_room == self.current_room {
	                	println!("The wumpus attacked you! You've been killed.");
	                	println!("That's what you get for trying to shoot at the mighty Wumpus");
	                    println!("Game Over!");
	                    self.player_alive = false;
	                    self.play_again();
	                } else {
	                	println!("The Wumpus moved to a new room.  Be careful");
	                }
				} else {
	                println!("The Wumpus ignores your pitiful attempt to shoot at him");
	            }
	        }
		} else {
			println!("You cannot shoot there.");
		}
	}

	fn inspect_current_room(&mut self) {
		println!("You arrive in room {} and look around", self.current_room);
		//check to see if wumpus lives there
	    if self.current_room == self.wumpus_room
	    {
	    	let rando: i32 = get_random(2);
	    	if rando == 0 {
	    		println!("You wake up the angry Wumpus");
	        	println!("The Wumpus ate you!!!");
	        	println!("LOSER!!!");
	        	println!("You die a painful death");
	        	self.player_alive = false;
	        	self.play_again();
	        } else {
	        	println!("You scared the wumpus so he runs away from you");
	        	self.move_startled_wumpus(self.wumpus_room);
	        }
	    }
	    //check for bats
	    else if self.current_room == self.bat1_room || self.current_room == self.bat2_room
	    {
	        let room_bats_left :i32= self.current_room;
	        let mut valid_new_bat_room :bool = false;
	        let mut is_current_room_bat_free : bool= false;
	        println!("Snatched by superbats!!");
	        if self.current_room == self.pit1_room || self.current_room == self.pit2_room {
	            println!("Luckily, the bats saved you from the bottomless pit!!")
	        }
	        
	        while !is_current_room_bat_free {
	            self.current_room =self.move_player(get_random(20));
	            println!("The bats move you to room {}", self.current_room);
	            if self.current_room != self.bat1_room && self.current_room != self.bat2_room {
	            	//break out of loop
	                is_current_room_bat_free = true;
	            } else {
	            	println!("The superbats snatch you again!!");
	            }
	        }
	        println!("The bats moved you to room {}", self.current_room);
	        self.inspect_current_room();

			//move bat1 if necessary
	        if room_bats_left == self.bat1_room {
	            while !valid_new_bat_room {
	                self.bat1_room = get_random(19) + 1;
	                println!("(bat1 moving to {}", self.bat1_room);
	                if self.bat1_room != self.wumpus_room && self.bat1_room != self.current_room {
	                    valid_new_bat_room = true;
	                }
	            }
	        }
	        //move bat2 if necessary
	        valid_new_bat_room = false;
	        if room_bats_left == self.bat2_room {
	        	while !valid_new_bat_room {
	                self.bat2_room = get_random(19) + 1;
	                println!("(bat2 moving to {}", self.bat2_room);
	                if self.bat2_room != self.wumpus_room && self.bat2_room != self.current_room {
	                    valid_new_bat_room = true;
	                }
	            }
	        }
	    }
	    //check for pits
	    else if self.current_room == self.pit1_room || self.current_room == self.pit2_room
	    {
	    	println!("You enter room {}", self.current_room);
	        println!("YYYIIIIIEEEEE.... fell in a pit!!!");
	        println!("GAME OVER LOSER!!!");
	        println!("You die a painful death");
	        self.player_alive = false;
	        self.play_again();
	    }
	    //move to new room
	    else
	    {
	        println!("You are in room {}", self.current_room);
	        if is_room_adjacent(self.current_room,self.wumpus_room) {
	            println!("You smell a horrid stench...");
	        }
	        if is_room_adjacent(self.current_room, self.bat1_room) || is_room_adjacent(self.current_room, self.bat2_room) {
	            println!("Bats nearby...");
	        }
	        if is_room_adjacent(self.current_room, self.pit1_room) || is_room_adjacent(self.current_room, self.pit2_room){
	            println!("You feel a draft...");
	        }
	        println!("Tunnels lead to rooms ");
	        for j in 0..3 
	        {
	        	let k :i32 = get_adjacent_room(self.current_room,j);
	            println!("{}", k);
	        }
	        //debugging
	        println!("(Psst. The wumpus is in room {} )", self.wumpus_room);
	    }
	}

	fn move_startled_wumpus(&mut self,room_num: i32){
		//rando is a number from 0..2
		let rando : i32= get_random(3);
		if rando <0 || rando>2 {
			println!("[move_startled_wumpus] rando = {}",rando);
		}
		self.wumpus_room = get_adjacent_room(room_num,rando);
	}

	// This restarts the map from the beginning without resetting the locations
	fn play_again(&mut self) {
		println!("Would you like to replay the same map? Enter 1 to play again.");
		let reply: i32 = input_number("?");
		if reply == 1 {
			self.current_room = self.starting_position;
			self.player_alive = true;
		    self.wumpus_room = self.wumpus_start;
		    self.bat1_room = self.bat1_start;
		    self.bat2_room = self.bat2_start;
		  
		    println!("Try not to die this time.");
		    self.inspect_current_room();
		    
		} else {
			self.player_alive = false;
			println!("Hahaha the Wumpus is laughing at you as you despair of ever beating him")
		}
	}

fn play_game(&mut self) {
	println!("Let the game of Hunt The Wumpus commence...");

  	// Initialize the game
	self.place_wumpus();
	self.place_bats();
	self.place_pits();
	self.place_player();

	// game set up
	self.player_alive = true;
	self.wumpus_alive = true;
	self.num_arrows = 3;

    //Inspects the initial room
    self.inspect_current_room();

    // Main game loop.
    while self.player_alive && self.wumpus_alive {
   		println!("Enter an action choice.");
    	println!("1) Move");
    	println!("2) Shoot");
    	println!("3) Quit");
    	println!("Please make a selection: ");
    	let mut choice:i32 = input_number("?");
    	let mut valid_choice :bool=false;
    	
		while !valid_choice {
	    	valid_choice = true;	        
	        match choice {
	        	1 => self.action_move(),
	        	2 => self.shoot(),
				3 => {
				    println!("Quitting the current game.");
        			self.player_alive = false;
        			break;
	               },
				_ => {	                  
	            	valid_choice = false;
	                println!("Invalid choice. Please try again.");
	                choice = input_number("?");
	            }
	    	} //end match
		} 
	}
}

fn start_game(&mut self) {
  	let mut keep_playing :bool = true;
  	
  	while keep_playing {
  		//the only way to break out of this endless loop is by entering 3 (quit)
      	println!("Welcome to Hunt The Wumpus.");
      	println!("1) Play Game");
      	println!("2) Print Instructions");
      	println!("3) Quit");
      	println!("Please make a selection: ");
      	let mut choice :i32  = input_number("?");
      	//println("(choice '${choice}')");
      	let mut valid_choice :bool = false;
      	
		while !valid_choice {
			//the only way to break out of this endless loop is by:
			//entering 3 (quit) or entering a valid number	
        	valid_choice = true;	//assume innocent until proven guilty
       		match choice {
       			1  => self.play_game(),
        	    2  =>  print_instructions(),
        	    3  => 	{
        	    		keep_playing = false;
              			println!("Quitting.");		
              			break;
           				},
           		_ => {  
           				valid_choice = false;
           				println!("Invalid choice. Please try again.");
           				choice = input_number("?");
           			}	
       		}	//end match 
		} 	//end !valid_choice
	}	//end keep playing
}	//end fn	
	
}

//----------------------------------------

fn main() {
	let mut w :WumpusGame = WumpusGame::new();
	w.start_game();
}