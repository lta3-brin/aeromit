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

Pengembangan ui dilakukan menggunakan [Next.js](https://nextjs.org/) dan dibuat melalui _cli_ [`create-next-app`](https://github.com/vercel/next.js/tree/canary/packages/create-next-app).

### Cara Memulai

Pertama, jalankan development server:

```bash
npm run dev
# or
yarn dev
```

Buka browser dengan alamat [http://localhost:3000](http://localhost:3000) untuk melihat halaman utama.

Modifikasi halaman misalkan `pages/index.js`. Halaman terkait akan otomatis diperbarui.

### Pelajari Selanjutnya

Untuk mempelajari lebih tentang Next.js, silahkan masuk ke halaman berikut ini:

- [Dokumentasi Next.js](https://nextjs.org/docs) - pelajari tentang fitur-fitur Next.js dan API.
- [Belajar Next.js](https://nextjs.org/learn) - seri belajar interaktif Next.js.
