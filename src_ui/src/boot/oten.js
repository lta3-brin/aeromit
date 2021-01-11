import {Cookies} from "quasar"
import {checkError} from "src/handlers/error";

// "async" is optional;
// more info on params: https://quasar.dev/quasar-cli/boot-files
export default ({ router, store }) => {
  router.beforeEach(async (to, from, next) => {
    const ketemu = to.matched.some(value => value.meta["kunci"])

    if (ketemu) {
      const token = Cookies.get("_msk")

      if (token) {
        try {
          await store.dispatch("otentikasi/checkOtenAction")
          next()
        } catch (err) {
          checkError(err, store)
          next({name: "masuk"})
        }
      } else {
        next({name: "masuk"})
      }
    } else {
      next()
    }
  })
}
