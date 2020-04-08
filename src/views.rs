use super::dating::Date;
use http::{uri, Uri};

///////////
// Views //
///////////

/// A View defines what to look at through which satellite, but not the date.
pub struct View {
   satellite: SatelliteLayer,
   lat_min: String,
   lat_max: String,
   lon_min: String,
   lon_max: String,
   width: String,
   height: String,
   image_type: ImageType,
}

#[derive(Debug)]
pub enum SatelliteLayer {
   Terra,
   Aqua,
   Sat(String), // should be checked before entry
}

#[derive(Debug)]
pub enum ImageType {
   JPEG,
   PNG,
}

//////////////////
// Making Views //
//////////////////

impl View {
   pub fn new(
      satellite: SatelliteLayer,
      lat_min: String,
      lat_max: String,
      lon_min: String,
      lon_max: String,
      width: String,
      height: String,
      image_type: ImageType,
   ) -> Self {
      View {
         satellite,
         lat_min,
         lat_max,
         lon_min,
         lon_max,
         width,
         height,
         image_type,
      }
   }
}

//////////////////////////////
// Turning things into URls //
//////////////////////////////

trait Query {
   fn query(&self) -> String;
   // no checks on this! Would be nice/proper but seems hardly necessary
}

pub fn locate(v: &View, d: &Date) -> Uri {
   // better to make builder and base once and reuse, no?
   let builder = get_default_uri();
   let base = get_base_pnq();
   let dateq = d.query();
   let viewq = v.query();
   let pathnq = format!("{}{}{}", base, dateq, viewq);

   // uri is builder+pnq, built and unwrapped.
   builder.path_and_query(pathnq.as_bytes()).build().unwrap()
}

impl Query for SatelliteLayer {
   fn query(&self) -> String {
      let layer_name = match self {
         SatelliteLayer::Terra => String::from("MODIS_Terra_correctedReflectance_TrueColor"),
         SatelliteLayer::Aqua => String::from("MODIS_Aqua_correctedReflectance_TrueColor"),
         SatelliteLayer::Sat(layer) => layer.to_string(), // TODO not sure why this works
      };
      format!("&LAYERS={}", layer_name)
   }
}

impl Query for ImageType {
   fn query(&self) -> String {
      let img_name = match self {
         ImageType::JPEG => String::from("JPEG"),
         ImageType::PNG => String::from("PNG"),
      };
      format!("&FORMAT=image/{}", img_name)
   }
}

impl Query for View {
   fn query(&self) -> String {
      format!(
         "{}{}&HEIGHT={}&WIDTH={}&BBox={},{},{},{}",
         self.satellite.query(),
         self.image_type.query(),
         self.height,
         self.width,
         self.lat_min,
         self.lon_min,
         self.lat_max,
         self.lon_max
      )
   }
}

impl Query for Date {
   fn query(&self) -> String {
      format!("&Time={}-{:02}-{:02}", self.y, self.m, self.d)
   }
}

////////////////////////////
// Producing a nearly-URL //
////////////////////////////

// could do this with static?
// pub fn get_default_uri() -> uri::Builder {
// pub fn get_default_uri() {
fn get_default_uri() -> uri::Builder {
   // returns the basic URI builder, for addition of queries.
   Uri::builder()
      .scheme("https")
      .authority("gibs.earthdata.nasa.gov")
}

fn get_base_pnq() -> String {
   // get the basic path and query
   let mut pnq = String::from("/wms/epsg4326/best/wms.cgi");
   pnq.push_str("?SERVICE=WMS&REQUEST=GetMap&VERSION=1.3.0&CRS=EPSG:4326");
   pnq
}

////////////////////////
// A view for testing //
////////////////////////

pub fn this_view() -> View {
   let sat = SatelliteLayer::Terra;
   let lat_min = String::from("28.582763671876");
   let lat_max = String::from("29.760498046876");
   let lon_min = String::from("82.357421875");
   let lon_max = String::from("83.717529296875");
   let width = String::from("619");
   let height = String::from("536");
   let img = ImageType::JPEG;

   View::new(sat, lat_min, lat_max, lon_min, lon_max, width, height, img)
}
