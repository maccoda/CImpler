#[macro_use]
extern crate log;
extern crate serde_yaml;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;
extern crate git2;

mod ci_build;
mod file_utils;
#[cfg(test)]
mod test_utils;


/// Error type for the conversion of the markdown files to the static site.
error_chain!{
    foreign_links {
        IO(std::io::Error);
        Config(serde_yaml::Error);
        Git(git2::Error);
    }

    errors {
        CmdFail(t: String)
        Fail(t: String)
        ScmFail(t:String)
    }
}

pub fn perform_build() -> Result<()> {
    let conf = ci_build::parse_user_build_file("tests/resources/test_config.yml")
        .expect("We have a bad file");
    println!("The configuration is {:?}", conf);
    // TODO Need to still work out how to obtain the information
    // let builder = ci_build::CiBuilder::new(BuildConfiguarion::new(
    // GitUrl("https://"),
    // GitCommit("master"),
    // conf,
    // ));
    // builder.exec_build()
    unimplemented!();
}
