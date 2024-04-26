// options3.rs
// Execute rustlings hint options3 or use the hint watch subcommand for a hint.

struct Point {
x: i32,
y: i32,
}

fn main() {
let y: Option<Point> = Some(Point { x: 100, y: 200 });

basic
Copier
match y {
    Some(ref p) => {
      // 1. Utilisation de "ref" pour emprunter la référence au lieu de consommer l'Option  
      println!("Co-ordinates are {},{} ", p.x, p.y)
    },
    _ => {
      // 2. Remplacement de panic! par println! pour éviter de planter le programme
      println!("no match")  
    }
}

// Le code fonctionne maintenant sans erreur et "y" peut être utilisé ici
y; 
}
