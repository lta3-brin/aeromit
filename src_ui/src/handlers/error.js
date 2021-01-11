const checkError = function (err, store) {
  let statusCode

  if (err.response) {
    statusCode = err.response.status
    store.commit('kegiatan/kegiatanMutation', err.response.data)
  } else {
    statusCode = 500
    store.commit('kegiatan/kegiatanMutation', {
      sukses: false,
      pesan: "Terjadi kesalahan yang perlu diperhatikan",
      hasil: err.message
    })
  }

  return statusCode
}

export {checkError}
