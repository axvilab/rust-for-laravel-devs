//! `'a` описує зв'язок: результат живе не довше за коротший з аргументів

fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}

fn main() {
    println!("{}", longer("Дніпро", "Київ")); // Дніпро

    let owned = String::from("Запоріжжя");
    let result;
    {
        let short = String::from("Суми");
        result = longer(&owned, &short);
        println!("{result}"); // ok: short ще живий
    }
    // println!("{result}"); // ПОМИЛКА E0597: `short` does not live long enough
}
