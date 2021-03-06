use seahorse::{App, Command, Context};

use std::env;

use base64rs::{encode, decode};

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("base64rs [args]")
        .action(default_action)
        .command(encode_command())
        .command(decode_command());

    app.run(args);
}

fn default_action(c: &Context) {
    if c.args.len() == 0 {
        c.help();
    } else {
        encode_action(c);
    }
}

fn encode_action(c: &Context) {
    println!("{}", encode(&c.args[0]));
}

fn encode_command() -> Command {
    Command::new("encode")
        .description("encode")
        .alias("e")
        .usage("base64 encode(e) {}")
        .action(encode_action)
}


fn decode_action(c: &Context) {
    println!("{}", decode(&c.args[0]));
}

fn decode_command() -> Command {
    Command::new("decode")
        .description("decode")
        .alias("d")
        .usage("base64 decode(d) {}")
        .action(decode_action)
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_encode() {
        let test_word = "rust";
        let res = encode(&test_word);

        // TODO: HashMapとかでお題と答えを持っておきたい
        assert_eq!(res, "cnVzdA==");
    }
}