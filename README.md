# :airplane: AEROMIT BBTA3 :rocket:
Repositori pengembangan experimental aeromit. Pengembangan perlu penambahan `.env` secara manual
didalam direktori utama aeromit ini dengan informasi sebagai berikut:

```
RUST_LOG=info
APP_ADDRESS=127.0.0.1:8080
APP_SALT=RASA_ASIN
APP_SECRET=super_secret
APP_EXPIRE=7
DEFAULT_DATABASE_NAME=<database_name>
DATABASE_URL=mongodb+srv://<USER>:<PASSWORD>@<HOST>/<database_name>?retryWrites=true&w=majority
```

## Pengembangan UI
Pengembangan UI dilakukan secara terpisah (_decouple_) dari sisi _backend_ dan terletak didalam
direktori `src_ui`.

Pengembangan ui dilakukan menggunakan [Quasar](https://quasar.dev/) dan dibuat melalui _cli_ 
[`quasar create .`](https://quasar.dev/quasar-cli/installation#Introduction).

### Cara Memulai

Pertama, jalankan development server:

```bash
quasar dev
```

Buka browser dengan alamat [http://localhost:8080](http://localhost:8080) untuk melihat halaman utama.

Modifikasi halaman misalkan `pages/Index.vue`. Halaman terkait akan otomatis diperbarui.
