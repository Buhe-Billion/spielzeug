fn
main
()
{
    let mut x: u128 = 7;
    println!("Der Wert von x in main() ist: {x}");
    x = 5;
    println!("Der Wert von x in main() ist: {x}");

    demo()
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
