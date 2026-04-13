mod read_and_write;

fn main() {
    let read_result = read_and_write::read();

    match read_result {
        Ok(_) => println!("Read data successfully"),
        Err(err) => println!("Error: {}", err),
    }

    let write_results = read_and_write::write();

    match write_results {
        Ok(_) => println!("Read data successfully"),
        Err(err) => println!("Error: {}", err),
    }
}
