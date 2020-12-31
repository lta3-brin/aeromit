import {goToHome} from "src/handlers/layout"
import InfoDialog from "src/components/InfoDialog/InfoDialog.vue"
import MenuSettings from "src/components/MenuSettings/MenuSettings.vue"

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
