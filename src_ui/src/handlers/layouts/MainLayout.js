import {goToHome} from "."
import InfoDialog from "components/layouts/InfoDialog"
import MenuSettings from "components/layouts/MenuSettings"

export default {
  name: 'MainLayout',
  data () {
    return {}
  },
  components: {
    InfoDialog,
    MenuSettings
  },
  methods: {
    goToHome,
    clicked: function() {
      this.$store.commit('layouts/openDialogMutation', true)
    }
  }
}
