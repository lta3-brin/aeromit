// "async" is optional;
// more info on params: https://quasar.dev/quasar-cli/boot-files
export default ({ router, store }) => {
  router.beforeEach(async (to, from, next) => {
    try {
      const res = await store.dispatch("otentikasi/checkOtenAction")

      console.log(res)

      next()
    } catch (err) {
      console.log(err.message)
      if (to.name === "masuk") {
        next()
      } else {
        next({name: "masuk"})
      }
    }
  })
}
