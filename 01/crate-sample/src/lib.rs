// lib.rs predstavlja korenski modul biblioteke, odnosno crate-a
mod example {
    // Ukoliko bi se ovde koristio pub mod, umesto dole pub use, poznavala bi se struktura modula, odnosno bilo bi moguće pozivati funkcije iz modula sa crate_sample::example::add_one
    /// Adds one to the number given.
    ///
    /// # Examples
    ///
    /// ```
    /// let arg = 5;
    /// let answer = crate_sample::example::add_one(arg);
    ///
    /// assert_eq!(6, answer);
    /// ```
    pub fn add_one(x: i32) -> i32 {
        // Ukoliko se pub ne navede, funkcija je vidljiva samo u ovom modulu
        x + 1
    }
}

mod example2 {
    use crate::example::add_one;
    /// Adds any number to the number given.
    ///
    /// # Examples
    ///
    /// ```
    /// let arg = 5;
    /// let num = 10;
    /// let answer = crate_sample::example2::add_any(arg, num);
    ///
    /// assert_eq!(15, answer);
    /// ```
    pub fn add_any(x: i32, y: i32) -> i32 {
        y + add_one(x) - 1
    }
}

pub use example::add_one; // Ovako se funkcija iz modula može koristiti izvan modula, na način da se poziva sa crate_sample::add_one, odnosno da se ne poznaje struktura modula
pub use example2::add_any;
