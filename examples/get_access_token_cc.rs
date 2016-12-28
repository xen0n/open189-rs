extern crate open189;


fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        println!("usage: {} <app id> <secret>", args[0]);
        std::process::exit(1);
    }

    let app_id = &args[1];
    let secret = &args[2];

    let app = open189::Open189App::new(app_id, secret);
    let result = app.get_access_token_cc();
    println!("result = {:?}", result);
}
