fn main() {
   println!("{:?}", geobird::this_uri());

   for u in geobird::these_uris() {
      println!("{:?}", u);
   }
}
