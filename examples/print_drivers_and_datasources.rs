use ibm_db::create_environment_v3;
use std::error::Error;

fn main() {

    match print_drivers_and_datasources() {
        Ok(()) => (),
        Err(err) => println!("{}", err),
    }
}

fn print_drivers_and_datasources() -> Result<(), Box<dyn Error>> {


    let mut env = create_environment_v3().map_err(|e| e.unwrap())?;

    println!("Driver list:");
    for driver_info in env.drivers()? {
        println!("\nDriver Name: {}", driver_info.description);
        for (key, value) in driver_info.attributes {
            println!("    {}={}", key, value);
        }
    }

    println!("\nDataSource list:");
    for ds in env.data_sources()? {
        println!("    {}\n    {}\n\n", ds.server_name, ds.driver);
    }
    Ok(())
}
