use std::io;
struct Shop{
    shop_type: String,
    opens: i32,
    closes: i32,
    open: bool,
}

impl Shop{

    //Used for creating a Shop
    fn getInstance(shop_type : String, opens: i32, closes: i32, open: bool ) -> Shop{

        Shop {shop_type: shop_type, opens: opens, closes: closes, open: open}

    }

    //Print out shop info
    fn printInfo(&self){
        println!("Shop Type:{}", self.shop_type);
        println!("Shop opens:{}", self.opens);
        println!("Shop closes:{}", self.closes);        
    }

    //Takes in time and decides if the shop is currently open
    //and saves it
    fn setOpen(&mut self, time: i32) -> bool{

        if time >= self.opens && time < self.closes{
            self.open = true;
        }
        else{
            self.open = false;
        }
        self.open
    }

    //Return bool off if shop is open
    fn isShopOpen(&self) -> bool{
        self.open
    }

}

fn main() {

    //Create example shops
    let shop1 = Shop::getInstance(String::from("Burger"), 11, 22, false);
    let shop2 = Shop::getInstance(String::from("Pizza"),11 ,22 , false);
    let shop3 = Shop::getInstance(String::from("Grocery"),8 ,23 , false);
    let shop4 = Shop::getInstance(String::from("Donut"),2 ,15 , false);
    let shop5 = Shop::getInstance(String::from("Hardware"),10 , 21, false);
    

    //Store shops in array which removes initial shops. 
    let mut downtown = [shop1, shop2, shop3, shop4, shop5];


    println!("Type when you want to shop:");


    let mut user_time = String::new();
    

    //Takes user input and stores in user_time
    io::stdin()
        .read_line(&mut user_time)
        .expect("User input eror!");

    println!("Shopping time: {}", user_time);


    //Convert string from user to 32 bit int
    let time = user_time.trim().parse::<i32>(). expect("Invalid input");


    //Loop through downton array comparing against shopping time given
    for i in 0..5 {

        downtown[i].setOpen(time);
        let is_open = downtown[i].isShopOpen();

        if is_open{
            println!("shop {} :{} is open", i, downtown[i].shop_type);
        }
        else{
            println!("shop {} :{} is closed.", i, downtown[i].shop_type);
        }
    }

    //Give info output for shops that are currently open. 
    println!("\n\nInfo for open shops:");

    for i in 0..5{
        if downtown[i].isShopOpen(){

            downtown[i].printInfo();
            println!("\n");
            
        }
    }

}
