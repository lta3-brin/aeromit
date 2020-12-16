import React from 'react'
import Topbar from "../components/topbar";

export default function DefaultLayout({children}) {
  return (
    <div>
      <Topbar />

      {children}
    </div>
  )
}
