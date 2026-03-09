use money::config::env::Config;

fn main() {
    Config::init();
    let _config = Config::get();
}
