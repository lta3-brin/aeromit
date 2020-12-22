const goToHome = function() {
  if (this.$route.path !== '/') {
    this.$router.push({name: 'utama'})
  }
}

export {goToHome}
