# Belajar Basic API dengan Rust Language
Repo ini dibuat semata-mata untuk penulis memperlancar menggunakan bahasa pemrograman Rust. Repo ini pastinya kurang sempurana karena dibuat apa adanya tanpa memiliki maksud dan tujuan khusus. 

## Pengaturan Database 
Untuk menjalankan server dengan repo ini, maka langkah pertama adalah pastikan database anda tersedia. Pada repo ini database yang digunakan adalah **PostgreSQL**.

Masukkan konfigurasi database pada file `.env`  
```
DATABASE_URL=postgres://<username>:<password>@<server>:<port>/<database>
```

## Migrasi
Setelah selesai melakukan konfigurasi database di file `.env` selanjutnya lakukan migrasi database melalui `terminal`.
```
diesel migration run
```

## Jalankan Server
Menjalankan server dilakukan melalui terminal. Gunakan perintah berikut :
```
cargo run
```
Setelah melakukan perintah diatas tunggu beberapa saat hingga server berjalan sempurana. Perintah diatas menjalankan server tanpa melihat perubahan yang terjadi.!

Gunakan perintah berbeda untuk menjalankan server dan melakukan pembaharuan otomatis (*Auto-Reloading*) jika terdapat file yang berubah. Tuliskan dalam `terminal`  perintah berikut :
```
cargo watch -x run
```
Pastikan sebelumnya telah menginstall `cargo-watch` sebelum menjalankan perintah diatas.

Server berjalan pada server [`http://localhost:8080`](http://localhost:8080)

## Dokumentasi API
Dokumentasi sudah tersedia dalam server anda dapat mengunjunginya dan melihat apa saja API yang ada pada server.
Penjelasan mengenai API sudah tersedia dalam dokumentasi meskipun kurang sempurna.

Dokumentasi dari API tersedia dengan 2 versi yakni `Swagger.ui` dan `redoc`. Silahkan kunjungi yang menurut anda lebih mudah. 

Untuk versi `Swagger` silahkan kunjungi server dengan link [`http://localhost:8080/docs/`](http://localhost:8080/docs/).

Sedangkan untuk versi `Redoc` silahkan kunjungi server dengan link [`http://localhost:8080/redoc`](http://localhost:8080/redoc).

## Unit Test
Unit test dibuat dengan menggunakan `actix_web::test`. Pembuatan test dibuat dengan **integration test** dimana file test berada diluar dari file kode src. Proses pembuatan test dibuat dengan membuat folder `tests` dan didalamnya terdapat file inti yakni `integration_test.rs`.

Test dalam repo ini belum sempurna karena keterbatasan pemahaman penulis dalam melakukan testing dan kurangnya rujukan untuk dijadikan bahan referensi. 
Sebelum menuliskan Unit testing perhatikan beberapa hal yakni :
1. Sebelumnya file dalam src belum ada file bernama `lib.rs`. Untuk melakukan integration_test perlu menggunakan file `lib.rs` dan modul yang dibuat dalam file src dideklarasikan difile ini.
2. Jika ingin menuliskan Unit test pastinya kita ingin mengakses modul dan fungsi yang ada di file src. Dengan mendeklarasikan modul di file `lib.rs` maka kita dapat mengimport dari folder test modul dan fungsi yang ada di src.
3. Gunakan `use <cargo_package_name>::<nama_modul>::<nama_fungsi>`. untuk mengimpor.
4. Jika unit test dipisah berdasarkan modul, maka pastikan untuk mendeklarasikan modul test dalan file `integration_test.rs`


## Note
Catatan ini dibuat untuk pengingat bagi penulis saja. Jika ada salah dalam pembahasan murni karena kurangnya pemahaman penulis dalam belajar.