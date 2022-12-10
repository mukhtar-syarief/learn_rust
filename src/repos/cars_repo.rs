use diesel::{prelude::*};

use crate::models::cars::{NewCars, Cars};

pub struct CarsRepo;


impl CarsRepo {
    pub fn get_all_type(conn: &mut PgConnection) -> Vec<Cars>{
        use crate::schema::cars::dsl::*;
    
        cars.order_by(id).load::<Cars>(conn).expect("Kesalahan koneksi.!")
    }
    
    pub fn create_new_car(conn: &mut PgConnection, car_type: &String) -> Cars {
        use crate::schema::cars;
    
        let new_cars = NewCars {
            type_: car_type
        };
    
        diesel::insert_into(cars::table)
            .values(&new_cars)
            .get_result(conn)
            .expect("Gagal menambah daftar tipe mobil")
    }
    
    pub fn delete_type(conn: &mut PgConnection, car_type: &String) {
        use crate::schema::cars::dsl::*;
        
        diesel::delete(cars.filter(type_.eq(car_type))).execute(conn).expect("Tipe mobil gagal dihapus");
    }
}