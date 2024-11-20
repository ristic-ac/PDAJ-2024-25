// Define an enum to represent different weather conditions
enum Weather {
    Sunny,
    Cloudy,
    Rainy(Option<u8>), // Optional intensity in percentage (0-100)
}

fn main() {
    // 1. Example of using an irrefutable pattern with `let`
    let today_weather = Weather::Rainy(Some(70)); // This will always match, so `let` works here

    // 2. Using `match` to handle all possible weather conditions with refutable patterns
    match today_weather {
        Weather::Sunny => {
            println!("It's sunny today! Enjoy the sunshine!");
        }
        Weather::Cloudy => {
            println!("It's cloudy. You might want to take a jacket.");
        }
        // `Rainy` with an optional intensity (refutable pattern)
        Weather::Rainy(Some(intensity)) => {
            println!("It's raining with an intensity of {}%!", intensity);
        }
        Weather::Rainy(None) => {
            println!("It's raining, but we don't know the intensity.");
        }
    }

    // 3. Using `if let` to handle a specific refutable pattern
    // Only print a message if it's raining with known intensity
    if let Weather::Rainy(Some(intensity)) = today_weather {
        println!("Make sure to carry an umbrella! Rain intensity: {}%", intensity);
    }

    // 4. Trying to use a refutable pattern in `let` (will cause an error if uncommented)
    // Uncommenting the following line will result in a compile error:
    // let Weather::Rainy(Some(intensity)) = today_weather;

    // 5. Correct approach with `if let` for refutable patterns
    if let Weather::Rainy(None) = today_weather {
        println!("It's raining, but no intensity info is available.");
    }

    // 6. An irrefutable `let` example
    let temperature = 25; // Always matches, so `let` works fine
    println!("Today's temperature is {}°C", temperature);
}
