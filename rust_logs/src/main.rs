use std::{fs, io::Error};

fn extract_errors(text: &str) -> Vec<&str> {
    // Gets text as read-only reference.
    // Then with split function creates vector of string slices
    // over the text back at the parent scope.
    let split_text = text.split("\n");

    // Created a vector.
    let mut result = vec![];

    // For each maching slice we passed these into result vector by
    // copying slices into strings to live outside of the scope in
    // parent scope.
    for line in split_text {
        if line.starts_with("ERROR") {
            result.push(line);
        }
    }

    // Passed the ownership of the result vector back at the parent scope.
    result
}
fn main() -> Result<(), Error> {
    // In this scope, we created error_logs inside this scope and
    // want to use value that has returned in Ok(){} scope.
    // We had to copy the matched statements into Vec<String> to do that.
    // But if we have multiple GB file, it would be waste of memory.
    // In that case, we may return Vec<&str> and not copy the text slices by defining
    // our algorithm in Ok(){} scope. Because text variable cannot live outside of the
    // Ok(){} scope.
    //let mut error_logs = vec![];

    // Messy code with nested matches
    // match fs::read_to_string("logs.txt") {
    //     Ok(text) => {
    //         let error_logs = extract_errors(text.as_str());

    //         match fs::write("errors.txt", error_logs.join("\n")) {
    //             Ok(..) => println!("Write Succesful !"),
    //             Err(err_message) => {
    //                 println!("Writing errors failed : {}", err_message)
    //             }
    //         }
    //     }
    //     Err(err) =>{
    //         println!("Error: {}", err)
    //     }
    // }

    // Quick way of handling errors for debugging.
    // let text = fs::read_to_string("logs.txt").expect("Failed to read logs.txt");
    // let error_logs = extract_errors(text.as_str());
    // fs::write("errors.log", error_logs.join("\n")).expect("Failed to write errors.txt");

    // We don't have meaningful way to handle error cases in this project.
    // We can use try operator to handle this errors.
    let text = fs::read_to_string("logs.txt")?;
    let error_logs = extract_errors(text.as_str());
    fs::write("errors.log", error_logs.join("\n"))?;

    Ok(())
}
