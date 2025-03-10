use std::io;

fn
main
()
{
    let mut x: u128 = 7;
    println!("Der Wert von x in main() ist: {x}");
    x = 5;
    println!("Der Wert von x in main() ist: {x}");

    demo();
    readFromFiveElementArray();
}

fn
demo
()
{
    let x = 11;
    let x = x + 2;

    {
        let x = (x * 2) - 3;
        println!("Der Wert von x im inneren Gültigkeitsbereich ist: {x}");
    }

    println!("Der Wert von x ist: {x}");
}

fn
readFromFiveElementArray
()
{
    let a:[u64;5] = [1,2,3,4,5];

    println!("Bitte gib einen Array-Index ein.");

    let mut index = String::new();

    io::stdin()
    .read_line(&mut index)
    .expect("Fehler beim Lesen der Zeile");

    let index: usize = index
    .trim()
    .parse()
    .expect("Eingegebener Index war keine Zahl");

    let element = a[index];
    println!("Der Wert von element beim Index {index} ist: {element}");
}
