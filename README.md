# Dapoer Papoea

**Dapoer Papoea** adalah aplikasi penyedia resep masakan khas Papua. Aplikasi ini menggunakan API REST berbasis Rust dengan versi API yang terstruktur dan menyajikan data resep serta bab dari file JSON.

## Fitur
- **API Versioning**: API menggunakan versi `v1` untuk memastikan kompatibilitas di masa depan.
- **Data Dinamis**: Data resep dan bab diambil dari file JSON (`recipes.json` dan `chapters.json`).
- **CRUD**: Mendukung operasi `GET` untuk mengambil data.

## Instalasi

### Prasyarat
Pastikan Anda telah menginstal:
- [Rust](https://www.rust-lang.org/tools/install)

### Langkah-langkah
1. Clone repository ini:
   ```bash
   git clone https://github.com/antheiz/dapoer-papoea-backend.git
   cd dapoer-papoea
   ```

2. Instal dependensi:

    ```
    cargo build
    ```

3. Jalankan aplikasi:

    ```
    cargo run
    ```

Aplikasi akan berjalan di http://127.0.0.1:8080.

### Lisensi

Proyek ini dilisensikan di bawah [MIT License](./LICENSE).