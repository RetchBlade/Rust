// Struktur für eine generische Liste
// T muss den Debug-Trait implementieren, damit wir die Elemente mit {:?} ausgeben können
struct MyList<T: std::fmt::Debug> {
    items: Vec<T>, // Hier wird ein Vektor (Vector) gespeichert, der generische Datentypen enthalten kann
}

// Implementierung von Methoden für MyList
impl<T: std::fmt::Debug> MyList<T> {
    // T muss den Debug-Trait implementieren
    // Konstruktor für MyList, der eine neue leere Liste erzeugt
    fn new() -> Self {
        MyList { items: Vec::new() }
    }

    // Methode zum Hinzufügen von Elementen zur Liste
    fn add(&mut self, item: T) {
        self.items.push(item); // Das Element wird an das Ende des Vektors hinzugefügt
    }

    // Methode zum Durchlaufen der Liste und Ausgeben der Elemente
    fn iterate(&self) {
        for item in &self.items {
            // Durch alle Elemente in der Liste iterieren
            println!("{:?}", item); // Ausgabe jedes Elements (hier brauchen wir den Debug-Trait)
        }
    }
}

// Trait zur Demonstration von objektorientierten Konzepten
trait Printable {
    // Definition einer Methode, die von Typen implementiert werden kann, die Printable sind
    fn print(&self);
}

// Implementierung des Printable-Traits für den Datentyp i32 (Ganzzahlen)
impl Printable for i32 {
    fn print(&self) {
        println!("Integer: {}", self); // Ausgabe der Ganzzahl
    }
}

// Implementierung des Printable-Traits für den Datentyp String
impl Printable for String {
    fn print(&self) {
        println!("String: {}", self); // Ausgabe des Strings
    }
}

// Funktion zum Demonstrieren von Ownership und Borrowing
fn ownership_and_borrowing() {
    let s1 = String::from("Rust Ownership"); // s1 ist der Besitzer des Strings
    let s2 = &s1; // Borrowing: s2 ist eine Referenz auf s1, aber besitzt den Wert nicht

    println!("Original String: {}", s1); // s1 kann noch verwendet werden, da es der Besitzer ist
    println!("Borrowed String: {}", s2); // s2 wird ausgeliehen, daher auch gültig

    // Ownership wird hier an s3 übergeben, und s1 kann danach nicht mehr verwendet werden
    let s3 = s1; // s1 übergibt den Besitz an s3. s1 ist jetzt ungültig (ownership transfer)

    // Der folgende Code würde zu einem Fehler führen, da s1 jetzt ungültig ist:
    // println!("Original String after ownership transfer: {}", s1); // Fehler!

    // s3 ist nun der Besitzer des Strings und kann diesen weiterhin nutzen
    println!("New Owner String: {}", s3);
}

// Demonstration von Verzweigungen und Ablaufkontrollen
fn check_value(value: i32) {
    // if-Anweisung, um zu prüfen, ob der Wert größer als 10 ist
    if value > 10 {
        println!("Value is greater than 10"); // Wenn der Wert größer als 10 ist
    } else {
        // Wenn der Wert 10 oder kleiner ist, wird der match-Block verwendet
        match value {
            1..=5 => println!("Value is between 1 and 5"), // Wert zwischen 1 und 5
            6..=10 => println!("Value is between 6 and 10"), // Wert zwischen 6 und 10
            _ => println!("Value is out of the defined range"), // Standardausgabe für Werte außerhalb des Bereichs
        }
    }
}

fn main() {
    // Demonstration von Ownership und Borrowing
    ownership_and_borrowing();

    // Demonstration von Generics und Iteratoren
    let mut list = MyList::<i32>::new(); // Erzeugt eine neue MyList, die Ganzzahlen speichert
    list.add(10); // Fügt die Zahl 10 hinzu
    list.add(20); // Fügt die Zahl 20 hinzu
    list.add(30); // Fügt die Zahl 30 hinzu
    list.iterate(); // Durchläuft die Liste und gibt die Zahlen aus

    // Demonstration von Traits und objektorientierten Konzepten
    let num = 42;
    num.print(); // Ruft die print-Methode für die Zahl 42 auf, die das Printable-Trait verwendet

    let text = String::from("Hello, Rust!");
    text.print(); // Ruft die print-Methode für den String auf, der das Printable-Trait verwendet

    // Demonstration von Verzweigungen und Ablaufkontrollen
    let value = 7;
    check_value(value); // Überprüft den Wert 7 und gibt entsprechende Nachricht aus
}
