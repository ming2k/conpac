mod install;

use clap::{Arg, Command, Error};
use std::fs::File;
use std::io::Read;
use serde_yaml;

mod package_struct;
use package_struct::Package;

fn install(package_name: &str) {
    let repo_path = "./pkg";
    let package_yaml_path = format!("{}/{}/package.yaml", repo_path, package_name);
    println!("Yaml path: {}", package_yaml_path);
    let mut file = File::open(package_yaml_path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    
    let package: Package = serde_yaml::from_str(&content).unwrap();
    // get uri
    let uri = &package.source[0].uri;
    println!("URI: {}", uri);
    // Add your install logic here
}

// fn load_package_from_yaml(path: &str) -> Result<Package, Box<dyn Error>> {
//     let mut file = File::open(path)?;
//     let mut content = String::new();
//     file.read_to_string(&mut content)?;
    
//     let package: Package = serde_yaml::from_str(&content)?;
    
//     Ok(package)
// }

fn main() {
    let matches = Command::new("conpac")
        .version("1.0")
        .author("Your Name")
        .about("A command-line package manager")
        .subcommand(
            Command::new("install").about("Install a package").arg(
                Arg::new("package_name")
                    .required(true)
                    .index(1)
                    .help("Name of the package to install"),
            ),
        )
        .subcommand(
            Command::new("uninstall").about("Uninstall a package").arg(
                Arg::new("package_name")
                    .required(true)
                    .index(1)
                    .help("Name of the package to uninstall"),
            ),
        )
        .subcommand(
            Command::new("upgrade").about("Upgrade a package").arg(
                Arg::new("package_name")
                    .required(true)
                    .index(1)
                    .help("Name of the package to upgrade"),
            ),
        )
        .subcommand(
            Command::new("query")
                .about("Query information about a package")
                .arg(
                    Arg::new("package_name")
                        .long("package")
                        .short('p')
                        .takes_value(true)
                        .required(true)
                        .help("Name of the package to query"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("install", sub_m)) => {
            let package_name = sub_m.value_of("package_name").unwrap();
            println!("Installing package: {}", package_name);
            install(&package_name);
            // Add your install logic here
        }
        Some(("uninstall", sub_m)) => {
            let package_name = sub_m.value_of("package_name").unwrap();
            println!("Uninstalling package: {}", package_name);
            // Add your uninstall logic here
        }
        Some(("upgrade", sub_m)) => {
            let package_name = sub_m.value_of("package_name").unwrap();
            println!("Upgrading package: {}", package_name);
            // Add your upgrade logic here
        }
        Some(("query", sub_m)) => {
            let package_name = sub_m.value_of("package_name").unwrap();
            println!("Querying package: {}", package_name);
            // Add your query logic here
        }
        _ => println!("Invalid command. Use 'conpac --help' for usage information."),
    }
}
