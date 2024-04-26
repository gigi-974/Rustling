// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.

// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a hint.

// Ajout du type générique <T> pour permettre à Wrapper de stocker n'importe quel type

struct Wrapper<T> {
    value: T,
}

// Ajout du type générique <T> pour l'implémentation des méthodes de Wrapper

impl<T> Wrapper<T> {
    // Modification de la signature de new pour accepter un argument value de type T
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        // Pas besoin de spécifier le type générique car u32 peut être inféré
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        // Spécification explicite du type générique &str car il ne peut pas être inféré
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
