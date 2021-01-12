import {Cookies} from "quasar"
import {goToPage} from "src/handlers/menu"


export default {
  name: 'MenuSettingsComponent',
  data () {
    return {}
  },
  methods: {
    goToPage,
    keluar() {
      Cookies.remove("_msk")
      this.$store.commit("otentikasi/tokenExistMutation", false)
      this.$router.push({name: "masuk"}).catch(_ => {})
    }
  },
  computed: {
    isSignIn: function () {
      return this.$store.getters["otentikasi/tokenExistGetter"]
    }
  }
}
