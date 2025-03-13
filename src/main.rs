use std::io;

impl Rectangle
{
	fn area(&self) -> u32 {self.width*self.height}
}


#[derive(Debug)]
struct Rectangle { width: u32, height: u32,}

fn
first_word
//this version can be used with both &String and &str values.
(ins: &str) -> &str
//this version below is less general
//(ins: &String) -> &str
{

	let byteRepresentation = ins.as_bytes();

	for (index, &item) in byteRepresentation.iter().enumerate()
	{
		if item == b' '
		{
			return &ins[..index];
		}
	}

	&ins[..]
}

fn
calcLen
(ins: String) -> (String,usize)
{
     let length = ins.len();
     (ins,length)
}

fn
calculate_length
(ins: &String) -> usize
{ins.len()}

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

    let s1 = String::from("hello");
    let (s2, len) = calcLen(s1);
    println!("The length of '{s2}' is {len}.");

    let len = calculate_length(&s2);
    println!("The length of '{s2}' is {len}.");

    let brownFox = String::from("The quick brown fox jumped over the lazy dogs");
    let erst = first_word(&brownFox);
    println!("Erst Wort ist : {erst}");

	let rect1 = Rectangle {width: 30, height: 50,};
	println!("The area of {rect1:?} is {}", rect1.area());
	dbg!(&rect1);	//	Please raed up on dbg and ownership
	println!("{rect1:?} Check out the #derive atributes. We have a lot to learn on that");
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
