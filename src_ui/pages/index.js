import React from "react"
import Head from "next/head"
import DefaultLayout from "../layouts/default"


export default function Home() {
  return (
    <div>
      <Head>
        <title>Aeromit BBTA3</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <DefaultLayout>
        <h1>Permulaan Aeromit UI</h1>
      </DefaultLayout>
    </div>
  )
}
