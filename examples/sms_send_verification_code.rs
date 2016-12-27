extern crate open189;


fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 6 {
        println!("usage: {} <app id> <secret> <access token> <phone> <code> <expire time>",
                 args[0]);
        std::process::exit(1);
    }

    let ref app_id = args[1];
    let ref secret = args[2];
    let ref access_token = args[3];
    let ref phone = args[4];
    let ref code = args[5];
    let expire_time: Option<usize> = if args.len() < 7 {
        None
    } else {
        Some(args[6].parse().unwrap())
    };

    let app = open189::Open189App::new(app_id, secret);
    let sms_token = app.sms_get_token(access_token);
    println!("sms token = {:?}", sms_token);
    let sms_token = sms_token.unwrap();

    let config = open189::SmsCodeConfig::prepared(phone, code, expire_time);

    let result = app.sms_send_verification_code(access_token, &sms_token, config);
    println!("send result = {:?}", result);
}
