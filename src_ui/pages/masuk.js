import React from "react"
import Head from "next/head"
import DefaultLayout from "../layouts/default";


export default function Masuk() {
  return (
    <div>
      <Head>
        <title>Silahkan Masuk - Aeromit BBTA3</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <DefaultLayout>
        <h1>Ini halaman masuk</h1>
      </DefaultLayout>
    </div>
  )
}
