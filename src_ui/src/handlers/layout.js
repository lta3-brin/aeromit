import {openURL} from 'quasar'

const goToHome = function() {
  if (this.$route.path !== '/') {
    this.$router.push({name: 'utama'})
  }
}

const goToExternal = function(url) {
  openURL(url, null, {noopener: true, noreferrer: true})
}

export {
  goToHome,
  goToExternal
}
