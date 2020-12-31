import {date} from 'quasar'

export default {
  name: 'ActivityPageComponent',
  data() {
    return {
      openDialog: false
    }
  },
  props: {
    data: Object
  },
  filters: {
    format(val) {
      return date.formatDate(val, 'DD MMMM YYYY, HH:mm')
    }
  }
}
