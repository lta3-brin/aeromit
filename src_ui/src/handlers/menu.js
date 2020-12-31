const goToPage = function(name) {
  this.$router.push({name}).catch(_ => {})
}

export {
  goToPage
}
