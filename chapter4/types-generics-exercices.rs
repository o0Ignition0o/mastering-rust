// types-generics-exercices.rs
use std::ops::{Add, AddAssign};

struct Money {
    amount: i32,
    currency: String,
}

impl AddAssign for Money {
    fn add_assign(&mut self, rhs: Money) {
        assert!(self.currency == rhs.currency);
        *self = Money {
            amount: self.amount + rhs.amount,
            currency: rhs.currency,
        }
    }
}

struct GenericMoney<T> {
    amount: T,
    currency: String,
}

impl<T: Add<T, Output = T>> Add for GenericMoney<T> {
    type Output = GenericMoney<T>;
    fn add(self, rhs: GenericMoney<T>) -> Self::Output {
        GenericMoney {
            currency: self.currency,
            amount: self.amount + rhs.amount,
        }
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    fn add(self, rhs: Point) {
        ((rhs.x + self.x) * (rhs.x + self.x) + (rhs.y + self.y) * (rhs.y + self.y)).sqrt()
    }
}

struct Square {
    top_left: Point,
    top_right: Point,
    bottom_left: Point,
    bottom_right: Point,
}

struct Rectangle {
    top_left: Point,
    top_right: Point,
    bottom_left: Point,
    bottom_right: Point,
}

pub trait Area {
    fn area(self) -> i32;
}

impl Area for Square {
    fn area(self) {
        self.top_right - self.top_left * self.top_right - self.bottom_right
    }
}
