use diesel::PgConnection;
use diesel::prelude::*;

use crate::models::location::{Location, Newlocation};


pub struct LocationRepo;


impl LocationRepo {
    pub fn get_all_location(conn: &mut PgConnection) -> Vec<Location> {
        use crate::schema::locations::dsl::*;
    
        let locs = locations.order_by(id).load::<Location>(conn).expect("Kesalahan pada server.");
        locs
    }
    
    pub fn create_new_loc(conn: &mut PgConnection, name_loc: &String) -> Location {
        use crate::schema::locations;
    
        let new_loc = Newlocation {
            region: &name_loc
        };
    
        diesel::insert_into(locations::table)
            .values(&new_loc)
            .get_result(conn)
                .expect("Gagal menambah lokasi.")
    }
    
    pub fn get_location(conn: &mut PgConnection, loc_id: &i32) -> Location {
        use crate::schema::locations::dsl::*;
    
        locations.filter(id.eq(loc_id)).first(conn).expect("Lokasi tidak ditemukan")
    }
    
    pub fn delete_location(conn: &mut PgConnection, loc_id: &i32) {
        use crate::schema::locations::dsl::*;
        
        diesel::delete(locations.filter(id.eq(loc_id))).execute(conn).expect("Gagal menghapus lokasi");
    }
}