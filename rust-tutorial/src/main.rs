use std::fs;

fn html2md_fun(url: &String, output: &String) {
    
    println!("Fetching url: {}", url);

    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("Converting html to markdown...");

    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes()).unwrap();
    println!("Converted markdown has been save in {:?}.", output);
}

// fn apply(value: i32, f: fn(i32) ->i32) -> i32 {
//     f(value)
// }

// fn square(x: i32) -> i32 {
//     x * x
// }

// fn cube(x: i32) -> i32 {
//     x * x * x
// }

// fn pi() -> f64 {
//     3.1415926
// }

// fn not_pi() {
//     3.1415926;
// }

// #[derive(Debug)]
// enum Gender {
//     Unspecified = 0,
//     Female = 1,
//     Male = 2,
// }
// #[derive(Debug, Copy, Clone)]
// struct UserId(u64);

// #[derive(Debug, Copy, Clone)]
// struct TopicId(u64);

// #[derive(Debug)]
// struct User {
//     id: UserId,
//     name: String,
//     gender: Gender,
// }

// #[derive(Debug)]
// struct Topic {
//     id: TopicId,
//     name: String,
//     owner: UserId,
// }

// #[derive(Debug)]
// enum Event {
//     Join((UserId, TopicId)),
//     Leave((UserId, TopicId)),
//     Message((UserId, TopicId, String)),
// }

// fn process_event(event: &Event) {
//     match event {
//         Event::Join((user_id, topic_id)) => {
//             println!("User {:?} joined topic {:?}", user_id, topic_id)
//         }
//         Event::Leave((user_id, topic_id)) => {
//             println!("User {:?} left topic {:?}", user_id, topic_id);
//         }
//         Event::Message((user_id, topic_id, message)) => {
//             println!(
//                 "User {:?} sent message '{:?}' to topic {:?}",
//                 user_id, message, topic_id
//             );
//         }
//     }
// }

// fn process_message(event:&Event){
//     println!("\nProcessing message event");
//     if let Event::Message((user_id, topic_id, message)) = event {
//         println!(
//             "User {:?} sent message '{:?}' to topic {:?}",
//             user_id, message, topic_id
//         );
//     }
// }

// fn fib_loop(n: u8) {
//     let mut a = 1;
//     let mut b = 1;
//     let mut i = 2u8;
//     loop {
//         let c = a + b;
//         a = b;
//         b = c;
//         i += 1;
//         println!("next val is: {}", b);

//         if i >= n {
//             break;
//         }
//     }
// }

// fn fib_while(n: u8) {
//     let (mut a, mut b, mut i) = (1, 1, 2);
//     while i < n {
//         let c = a + b;
//         a = b;
//         b = c;
//         i += 1;
//         println!("next val is: {}", b);
//     }
// }

// fn fib_for(n: u8) {
//     let (mut a, mut b) = (1, 1);
//     for _ in 2..n {
//         let c = a + b;
//         a = b;
//         b = c;
//         println!("next val is: {}", b);
//     }
// }

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    html2md_fun(&args[1], &args[2]);
    // println!("apply square: {}", apply(5, square));
    // println!("apply cube: {}", apply(5, cube));

    // let is_pi = pi();
    // let is_unit1 = not_pi();
    // let is_unit2 = {
    //     pi();
    // };

    // println!("is_pi: {:?}, is_unit1: {:?}, is_unit2: {:?}", is_pi, is_unit1, is_unit2);

    // let alice = User {
    //     id: UserId(1),
    //     name: "Alice".into(),
    //     gender: Gender::Female,
    // };
    // let bob = User {
    //     id: UserId(2),
    //     name: "Bob".into(),
    //     gender: Gender::Male,
    // };

    // let topic = Topic {
    //     id: TopicId(1),
    //     name: "rust".into(),
    //     owner: UserId(1),
    // };

    // let event1 = Event::Join((alice.id, topic.id));
    // let event2 = Event::Join((bob.id, topic.id));
    // let event3 = Event::Message((alice.id, topic.id, "Hello World!".into()));

    // println!(
    //     "event1: {:?}, event2: {:?}, event3: {:?}",
    //     event1, event2, event3
    // );

    // process_event(&event1);
    // process_event(&event2);
    // process_event(&event3);

    // process_message(&event3);
    // let n = 10;
    // fib_loop(n);
    // fib_while(n);
    // fib_for(n);

    // println!("args0: {:?}", &args[0]);
    // println!("args1: {:?}", &args[1]);
    // println!("args2: {:?}", &args[2]);
}
