use std::fmt;
use std::ops::{Add, AddAssign};

///////////
// Dates //
///////////

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Date {
   pub y: i32,
   pub m: i32,
   pub d: i32,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Period {
   pub y: i32,
   pub m: i32,
   pub d: i32,
}

// #[derive(Debug)]
pub struct Dates {
   now: Date,
   end: Date,
   skip: Period,
}

impl Dates {
   pub fn new(now: Date, end: Date, skip: Period) -> Self {
      Dates { now, end, skip }
   }
}

impl Date {
   pub fn new(y: i32, m: i32, d: i32) -> Self {
      Date { y, m, d }
   }
}

///////////////////
// Numerical ops //
///////////////////

// Adding to a date first adds years, then months, then days.
impl Add<Period> for Date {
   type Output = Self;

   fn add(self, p: Period) -> Self::Output {
      // resolve years and months, then postpone by days
      let mut y = self.y + p.y + (self.m + p.m - 1) / 12;
      let mut m = (self.m + p.m - 1) % 12 + 1;
      let mut d = self.d + p.d;
      let mut dmax = 0;
      // Closures getting messy with borrow checking.
      // let update_month = || {
      //    // Debug to check new m captured:
      //    println!("check: month is {}", m);
      //    dmax = match m {
      //       // maximum days this month
      //       2 if is_leap(self.y) => 29,
      //       // i => MONTH_LENGTH[usize::From(i) - 1],
      //       i => month_length(i),
      //    }
      // };

      fn update_max(max: &mut i32, year: i32, month: i32) {
         *max = match month {
            // maximum days this month
            2 if is_leap(year) => 29,
            // i => MONTH_LENGTH[usize::From(i) - 1],
            i => month_length(i),
         }
      }

      // update_month();
      update_max(&mut dmax, y, m);

      while d > dmax {
         // shift up a month of days
         d -= dmax;
         m += 1;

         // overflow months
         if m > 12 {
            y += (m - 1) / 12;
            m = (m - 1) % 12 + 1;
         }
         // update_month();
         update_max(&mut dmax, y, m);
      }

      Date { y, m, d } // fail bc mut -> non-mut?
   }
}

impl AddAssign<Period> for Date {
   fn add_assign(&mut self, p: Period) {
      *self = *self + p; //TODO Understand pointer dereferencing!!
   }
}

impl Add for Period {
   type Output = Self;

   fn add(self, other: Period) -> Self::Output {
      Self {
         y: self.y + other.y,
         m: self.m + other.m,
         d: self.d + other.d,
      }
   }
}

// const MONTH_LENGTH: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
fn month_length(m: i32) -> i32 {
   match m {
      1 => 31,
      2 => 28,
      3 => 31,
      4 => 30,
      5 => 31,
      6 => 30,
      7 => 31,
      8 => 31,
      9 => 30,
      10 => 31,
      11 => 30,
      12 => 31,
      _ => panic!("What sort of a month is that {}?!!", m),
   }
}

fn is_leap(year: i32) -> bool {
   if year % 400 == 0 {
      true
   } else if year % 100 == 0 {
      false
   } else if year % 4 == 0 {
      true
   } else {
      false
   }
}

///////////////
// Iteration //
///////////////

impl Iterator for Dates {
   type Item = Date;
   fn next(&mut self) -> Option<Self::Item> {
      if self.now > self.end {
         None
      } else {
         let now = self.now; // or drop 'now' back in new()
         self.now += self.skip;
         Some(now)
      }
   }
}

/////////////
// Display //
/////////////

impl fmt::Display for Date {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{}-{:02}-{:02}", self.y, self.m, self.d)
   }
}

impl fmt::Debug for Date {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "date {}", self)
   }
}

impl fmt::Display for Period {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{}y{}m{}d", self.y, self.m, self.d)
   }
}

impl fmt::Debug for Period {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{}", self)
   }
}

///////////////////
// Example dates //
///////////////////

pub fn this_date() -> Date {
   Date::new(2019, 04, 01)
}

pub fn that_date() -> Date {
   Date::new(2020, 04, 01)
}

pub fn this_period() -> Period {
   Period { y: 0, m: 1, d: 5 }
}

pub fn these_dates() -> Dates {
   Dates::new(this_date(), that_date(), this_period())
}
