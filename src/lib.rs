mod dating;
// use dating::{Date, Dates, Period};
// use dating::Date;
mod views;

use http::uri::Uri;
pub use views::locate;

pub fn this_uri() -> Uri {
   locate(&views::this_view(), &dating::this_date())
}

pub fn these_uris() -> Vec<Uri> {
   let v = views::this_view();
   dating::these_dates().map(|d| locate(&v, &d)).collect()
}
