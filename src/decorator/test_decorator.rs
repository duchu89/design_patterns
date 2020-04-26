mod decorator;
mod pizza;

use decorator::{Cheese, Chicken};
use pizza::{Margherita, Pizza, Vegetariana};

pub fn test() {
    let margherita = Margherita;
    let margherita_with_cheese = Cheese {
        pizza: Box::new(margherita),
    };

    println!(
        "pizza type: {}, cost: {} $",
        margherita_with_cheese.get_description(),
        margherita_with_cheese.get_price()
    );

    let vegetariana_with_chicken_and_cheese = Chicken {
        pizza: Box::new(Cheese {
            pizza: Box::new(Vegetariana),
        }),
    };

    println!(
        "pizza type: {}, cost: {} $",
        vegetariana_with_chicken_and_cheese.get_description(),
        vegetariana_with_chicken_and_cheese.get_price()
    );
}
