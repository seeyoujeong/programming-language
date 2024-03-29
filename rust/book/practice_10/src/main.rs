fn main() {
    println!("Practice 10.0");
    practice_10_0();
    println!("---------------------------");

    println!("Practice 10.1");
    practice_10_1();
    println!("---------------------------");

    println!("Practice 10.2");
    practice_10_2();
    println!("---------------------------");

    println!("Practice 10.3");
    practice_10_3();
    println!("---------------------------");
}

fn practice_10_0() {
    fn largest<T>(list: &[T]) -> &T 
        where T: PartialOrd
    {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let numbers = vec![23, 43, 45, 19, 21];

    let result = largest(&numbers);
    println!("The largest number is {}", result);
}

fn practice_10_1() {
    #[derive(Copy, Clone)]
    struct Point<T, U> {
        x: T,
        y: U,
    }

    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 1.0, y: 4.0 };
    let p3 = Point { x: 5, y: 4.0 };
    let p4 = Point { x: "Hello", y: 'c' };

    impl<T, U> Point<T, U> {
        fn x(&self) -> &T {
            &self.x
        }

        fn mixup<V, W>(&self, other: Point<V, W>) -> Point<&T, W> {
            Point {
                x: &self.x,
                y: other.y,
            }
        }
    }

    let mix = p1.mixup(p4);

    println!("mix.x = {}, mix.y = {}", mix.x, mix.y);
    println!("&p1.x = {}", p1.x());
    println!("p1.x = {}", p1.x);
    println!("p4.y = {}", p4.y);
}

fn practice_10_2() {
    extern crate practice_10;

    use practice_10::Tweet;
    use practice_10::NewsArticle;
    use practice_10::Summarizable;

    let tweet = Tweet {
        username: String::from("jeong"),
        content: String::from("jeong is ..."),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summary());

    let article = NewsArticle {
        headline: String::from("..."),
        location: String::from("..."),
        author: String::from("jeong"),
        content: String::from("..."),
    };

    println!("New article available! {}", article.summary());

    struct WeatherForecast {
        high_temp: f64,
        low_temp: f64,
        chance_of_precipitation: f64,
    }

    impl Summarizable for WeatherForecast {
        fn author_summary(&self) -> String {
            String::from("...")
        }

        fn summary(&self) -> String {
            format!("The high will be {}, and the low will be {}. The chance of precipitation is {}%.", 
            self.high_temp, self.low_temp, self.chance_of_precipitation)
        }
    }

    let wheater_forecast = WeatherForecast {
        high_temp: 0.1,
        low_temp: -10.2,
        chance_of_precipitation: 40.6,
    };

    println!("{}", wheater_forecast.summary());
}

fn practice_10_3() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("Call me jeong. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };

    println!("{}", i.part);

    let s: &'static str = "I have a static lifetime.";

    println!("s: {}", s);

    use std::fmt::Display;

    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
        where T: Display
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let test = "test";
    let result = longest_with_an_announcement(string1.as_str(), string2, test);   
    println!("longest_with_an_announcement result: {}", result);
}