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