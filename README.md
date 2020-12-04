# :airplane: AEROMIT BBTA3 :rocket:
Repositori pengembangan experimental aeromit. Pengembangan perlu penambahan `.env` secara manual
didalam direktori utama aeromit ini dengan informasi sebagai berikut:

```
RUST_LOG=info
APP_ADDRESS=127.0.0.1:8080
APP_SALT=RASA_ASIN
DEFAULT_DATABASE_NAME=<database_name>
DATABASE_URL=mongodb+srv://<USER>:<PASSWORD>@<HOST>/<database_name>?retryWrites=true&w=majority
```
