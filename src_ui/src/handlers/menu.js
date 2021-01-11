import {openURL} from 'quasar'


const goToPage = function(name) {
  this.$router.push({name}).catch(_ => {})
}

const goToExternal = function(url) {
  openURL(url, null, {noopener: true, noreferrer: true})
}

export {
  goToPage,
  goToExternal
}
