#[macro_use]
extern crate log;
extern crate serde_yaml;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;

mod requests;
mod ci_build;
mod file_utils;
#[cfg(test)]
mod test_utils;


/// Error type for the conversion of the markdown files to the static site.
error_chain!{
    foreign_links {
        IO(std::io::Error);
        Config(serde_yaml::Error);
    }

    errors {
        CmdFail(t: String)
        Fail(t: String)
    }
}

pub fn perform_build() {
    // TODO Actualy make this useful
    let conf = ci_build::parse_build_file("tests/resources/test_config.yml")
        .expect("We have a bad file");
    println!("The configuration is {:?}", conf);
}
