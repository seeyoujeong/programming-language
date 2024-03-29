fn main() {
    println!("functions");
    functions();
    println!("-------------------------");

    println!("associated functions and methods");
    associated_functions_and_methods();
    println!("-------------------------");

    println!("closures");
    closures();
    println!("-------------------------");

    println!("higher order functions");
    higher_order_functions();
    println!("-------------------------");

    println!("diverging functions");
    diverging_functions();
    println!("-------------------------");
}

fn functions() {
    fizzbuzz_to(100);

    fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
        if rhs == 0 {
            return false;
        }

        lhs % rhs == 0
    }

    fn fizzbuzz(n: u32) -> () {
        if is_divisible_by(n, 15) {
            println!("fizzbuzz");
        } else if is_divisible_by(n, 3) {
            println!("fizz");
        } else if is_divisible_by(n, 5) {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // when a function returns '()', the return type can be omitted from the signature.
    fn fizzbuzz_to(n: u32) {
        for n in 1..=n {
            fizzbuzz(n);
        }
    }
}

fn associated_functions_and_methods() {
    struct Point {
        x: f64,
        y: f64,
    }

    impl Point {
        // associated function
        fn origin() -> Point {
            Point { x: 0.0, y: 0.0 }
        }

        fn new(x: f64, y: f64) -> Point {
            Point { x: x, y: y }
        }
    }

    struct Rectangle {
        p1: Point,
        p2: Point,
    }

    impl Rectangle {
        // method
        // '&self' is sugar for 'self: &Self', where 'Self' is the type of the 
        // caller object. In this case 'Self' = 'Rectangle'.
        fn area(&self) -> f64 {
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;

            ((x1 - x2) * (y1 - y2)).abs()
        }

        fn perimeter(&self) -> f64 {
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;

            2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
        }

        // '&mut self' desugars to 'self: &mut Self'.
        fn translate(&mut self, x: f64, y: f64) {
            self.p1.x += x;
            self.p2.x += x;

            self.p1.y += y;
            self.p2.y += y;
        }
    }

    struct Pair(Box<i32>, Box<i32>);

    impl Pair {
        // this method consumes the resources of the caller object.
        // 'self' desugars to 'self: Self'.
        fn destroy(self) {
            let Pair(first, second) = self;

            println!("Destroying Pair({}, {})", first, second);

            // 'first' and 'second' go out of scope and get freed.
        }
    }

    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // 'rectangle.perimeter()' === 'Rectangle::perimeter(&rectangle)'
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();
}

fn closures() {
    let outer_var = 42;

    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    let closure_inferred = |i| i + outer_var;

    println!("closure_annotated: {}", closure_annotated(1));
    println!("closure_inferred: {}", closure_inferred(1));
    // once closure's type has been inferred, it cannot be inferred again with another type.

    let one = || 1;
    println!("closure returning one: {}", one());

    println!("capturing");

    let color = String::from("green");

    let print = || println!("'color': {}", color);

    print();

    let _reborrow = &color;
    print();

    let _color_moved = color;

    let mut count = 0;

    let mut inc = || {
        count += 1;
        println!("'count': {}", count);
    };

    inc();

    // an attempt to reborrow will lead to an error.
    // let _reborrow = &count;
    inc();

    let _count_reborrowed = &mut count;

    use std::mem;

    // a non-copy type
    let movable = Box::new(3);

    let consume = || {
        println!("'movable': {:?}", movable);
        mem::drop(movable);
    };

    // 'consume' consumes the variable so this can only be called once.
    consume();
    // consume();

    let haystack = vec![1, 2, 3];

    // using move before vertical pipes forces closure to take ownership of captured variables.
    let contains = move |needle| haystack.contains(needle);

    println!("haystack contains 1? {}", contains(&1));
    println!("haystack contains 4? {}", contains(&4));

    println!("as input parameters");

    fn apply<F>(f: F) 
        where F: FnOnce() {
        f();
    }

    fn apply_to_3<F>(f: F) -> i32 
        where F: Fn(i32) -> i32 {
        f(3)
    }

    let greeting = "hello";

    // 'to_owned' creates owned data from borrowed one
    let mut farewell = "goodbye".to_owned();

    // capture 2 variables
    let diary = || {
        // requires 'Fn'
        println!("I said {}.", greeting);

        // requires 'FnMut'
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // requires 'FnMut'
        mem::drop(farewell);
    };

    apply(diary);

    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));

    println!("input functions");

    fn call_me<F: Fn()>(f: F) {
        f();
    }

    fn function() {
        println!("I'm a function!");
    }

    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);

    println!("as output parameters");

    fn create_fn() -> impl Fn() {
        let text = "Fn".to_owned();

        move || println!("This is a: {}", text)
    }

    fn create_fnmut() -> impl FnMut() {
        let text = "FnMut".to_owned();

        move || println!("This is a: {}", text)
    }

    fn create_fnonce() -> impl FnOnce() {
        let text = "FnOnce".to_owned();

        move || println!("This is a: {}", text)
    }

    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();

    println!("Iterator::any");

    // trait Iterator {
    //     type Item;

    //     fn any<F>(&mut self, f: F) -> bool 
    //         where F: FnMut(Self::Item) -> bool;
    // }

    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    // 'iter()' only borrows 'vec1'
    println!("vec1 len: {}", vec1.len());
    println!("First element of vec1 is: {}", vec1[0]);

    // 'into_iter()' does move 'vec2'
    // println!("First element of vec2 is: {}", vec2[0]);
    // println!("vec2 len: {}", vec2.len());

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    println!("2 in array2: {}", array2.into_iter().any(|x| x == 2));

    println!("searching through iterators");

    // trait Iterator {
    //     type Item;

    //     fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
    //         where P: FnMut(&Self::Item) -> boo;
    // }

    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    let mut iter = vec1.iter();
    let mut into_iter = vec2.into_iter();

    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
    println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    println!("Find 2 in array1: {:?}", array1.iter().find(|&&x| x == 2));
    println!("Find 2 in array2: {:?}", array2.into_iter().find(|&x| x == 2));

    let vec = vec![1, 9, 3, 3, 13, 2];

    // 'position()' does not take a reference.
    let index_of_first_even_number = vec.iter().position(|&x| x % 2 == 0);
    assert_eq!(index_of_first_even_number, Some(5));

    let index_of_first_negative_number = vec.into_iter().position(|x| x < 0);
    assert_eq!(index_of_first_negative_number, None);
}

fn higher_order_functions() {
    fn is_odd(n: u32) -> bool {
        n % 2 == 1
    }

    println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 1000;

    let mut acc = 0;

    for n in 0.. {
        let n_squared = n * n;

        if n_squared >= upper {
            break;
        } else if is_odd(n_squared) {
            acc += n_squared;
        }
    }
    println!("imperative style: {}", acc);

    let sum_of_squared_odd_numbers: u32 = 
        (0..).map(|n| n * n)
             .take_while(|&n_squared| n_squared < upper)
             .filter(|&n_squared| is_odd(n_squared))
             .sum();
    
    println!("functional style: {}", sum_of_squared_odd_numbers);
}

fn diverging_functions() {
    
    #[allow(dead_code)]
    fn foo() -> ! {
        panic!("This call never returns.");
    }

    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;

        for i in 0..up_to {
            let addition: u32 = match i % 2 == 1 {
                true => i,
                false => continue,
            };
            acc += addition;
        }

        acc
    }

    println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
}
