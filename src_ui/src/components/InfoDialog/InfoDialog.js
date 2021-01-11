import {goToExternal} from "src/handlers/menu"

export default {
  name: 'DialogComponent',
  data () {
    return {}
  },
  computed: {
    openDialog: {
      get() {
        return this.$store.getters['layouts/openDialogGetter']
      },
      set(val) {
        this.$store.commit('layouts/openDialogMutation', val)
      }
    }
  },
  methods: {
    goToExternal
  }
}
