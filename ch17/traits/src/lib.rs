#![allow(unused_variables)]

// Defining a Trait for Common Behavior
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // This vector is of type Box<dyn Draw>, which is a trait object; it’s a
    // stand-in for any type inside a Box that implements the Draw trait.
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// alternative struct with generic type parameter with trait bounds
// If you’ll only ever have homogeneous collections, using generics and trait
// bounds is preferable because the definitions will be monomorphized at
// compile time to use the concrete types.

// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }

// impl<T> Screen<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

// implementing the Trait
impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}
