use std::{fs, fs::File, io::{Write, BufWriter, BufReader},};

fn write_data_into_file(path: &str, data: &str) -> std::io::Result<()> {

    let file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;

    let mut file = BufWriter::new(file);

    //trying to write everything in one go
    file.write_all(&data.as_bytes())?;

    file.flush()?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let file = File::open("data.txt")?;
    let reader = BufReader::new(file);

    let deserializer = serde_json::Deserializer::from_reader(reader);
    let iterator = deserializer.into_iter::<serde_json::Value>();
    for item in iterator {
        let json_item = item?;
        println!("Got {:#?}", json_item);

        let mut json_string = serde_json::to_string_pretty(&json_item)?;
        json_string.push_str(&"\n");
        // using the writing function we created
        match write_data_into_file(&".\\test.log", &json_string) {
            Ok(_) => println!("Sucessfully wrote in the file!"),
            Err(e) => println!("Something went wrong opsies :< {:?}", e)
        }

    }   

    Ok(())
}