mod value_range;
use value_range::ValueRange;

use rand::{
    Rng,
    distributions::{Distribution, Standard}
};

fn main() {
    let max_planets = 10;
    let mut rng = rand::thread_rng();

    let stellar_class: StellarClass = rand::random();
    let star = StellarClassConfiguration::from(stellar_class);

    let planet_count = rng.gen_range(0..max_planets);
    let planets = 0..planet_count;

    println!("Star-Type: {}", star.name);
    for planet in planets {
        println!("Planet {}", planet)
    }

    println!("");
}

struct Star {
    name: String,
    stellar_class: StellarClassConfiguration,
    effective_temp_k: i32,
    mass_M: i32
}

enum StellarClass {
    O,
    B,
}

impl Distribution<StellarClass> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> StellarClass {
        match rng.gen_range(0..=1) {
            0 => StellarClass::O,
            1 => StellarClass::B,
            _ => StellarClass::B
        }
    }
}

struct StellarClassConfiguration {
    name: String,
    class: StellarClass,
    effective_temp_k: ValueRange<i32>,
    mass_M: ValueRange<f64>,
}

impl StellarClassConfiguration {
    fn from(class: StellarClass) -> StellarClassConfiguration {
        match class {
            StellarClass::B => StellarClassConfiguration::blue(),
            StellarClass::O => StellarClassConfiguration::blue_white()
        }
    }

    fn blue() -> StellarClassConfiguration {
        StellarClassConfiguration {
            name: String::from("blue"),
            class: StellarClass::O,
            effective_temp_k: ValueRange::from(30000),
            mass_M: ValueRange::from(16.0)
        }
    }

    fn blue_white() -> StellarClassConfiguration {
        StellarClassConfiguration {
            name: String::from("blue white"),
            class: StellarClass::B,
            effective_temp_k: ValueRange::fromTo(10000, 30000),
            mass_M: ValueRange::fromTo(2.1, 16.0)
        }
    }

    fn println(&self) {
        println!("St")
    }
}
