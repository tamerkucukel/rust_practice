// We can use &Vec<String> in here, but it has lack of flexibility.
// If we write that way, we cannot pass slice of a vector in this function.
// By doing this, we can easily print elements of a some portion of vector.
fn print_elements(elements: &[String]) {
    elements.iter()
            .map(|el| format!("{} {}", el, el))
            .for_each(|element| println!("{}", element));
}

// We did in place operation on elements of the mutable slice.
// Used iter_mut to change elements in place with truncate.
fn shorten_strings(elements: &mut [String], len: usize) {
    elements.iter_mut()
            .for_each(|el| el.truncate(len));
}

// We make operation on each "read" value of elements in elements slice.
// Then we collected them into Vec<String>
// collect generates Vec<String> by looking the return type of the function.
// We may use Turbofish to state creation of Vec<String> type.
// ex. .collect::<Vec<String>>()
fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements.iter()
            .map(|el| el.to_uppercase())
            .collect()
}

// into_iter() takes ownership of each value in vec_a.
// Therefore we able to push these values into vec_b.
// We don't need ownership of vec_b, it just need to be mutable.
fn move_elements(vec_a: Vec<String>, vec_b: &mut Vec<String>) {
    vec_a
        .into_iter()
        .for_each(|el|vec_b.push(el));
}

fn explode(elements: &[String]) -> Vec<Vec<String>>{
    elements.iter()
            .map(|el| el.chars().map(|c|c.to_string()).collect())
            .collect()

}

fn find_color_or(elements: &[String], search: &str, fallback: &str) -> String{
    elements.iter()
            .find(|el| el.contains(search))
            .map_or(String::from(fallback), |el|el.to_string())
}
fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    let uppercase_colors = to_uppercase(&mut colors);
    println!("{:#?}", uppercase_colors);

    let exploded = explode(&colors);
    println!("{:#?}", exploded);

    let found = find_color_or(&colors, "re", "orange");
    println!("{:#?}", found)
}
