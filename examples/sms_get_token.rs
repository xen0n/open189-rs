extern crate open189;


fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 4 {
        println!("usage: {} <app id> <secret> <access token>", args[0]);
        std::process::exit(1);
    }

    let app_id = &args[1];
    let secret = &args[2];
    let access_token = &args[3];

    let app = open189::Open189App::new(app_id, secret);
    let result = app.sms_get_token(access_token);
    println!("result = {:?}", result);
}
