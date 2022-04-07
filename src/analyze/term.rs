use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Term {
    Type(Type),
    Marker(Marker),
    App(App),
    // Trait(Trait),
}