mod content;
use content::media::Media;
use content::catalog::Catalog;

fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("An Audiobook")
    };
    let movie = Media::Movie {
        title: String::from("Good Movie"),
        director: String::from("Good Author")
    };
    let book = Media::Book {
        title: String::from("Bad Book"),
        author: String::from("Bad Author")
    };

    let podcast = Media::Podcast(10);
    let placeholder = Media::Placeholder;

    // println!("{}", audiobook.description());
    // println!("{}", movie.description());
    // println!("{}", book.description());

    let mut catalog = Catalog::new();

    catalog.add(audiobook);
    catalog.add(movie);
    catalog.add(book);
    catalog.add(podcast);
    catalog.add(placeholder);

    let item = catalog.get_by_index(40);
    

    println!("{:#?}",item.unwrap_or( &Media::Audiobook { title: String::from("123123") } ))

}
 