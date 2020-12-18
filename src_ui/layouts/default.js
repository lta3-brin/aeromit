import React from "react"
import Topbar from "../components/topbar"
import {Container} from "@material-ui/core"


export default function DefaultLayout({children}) {
  return (
    <div>
      <Topbar />

      <Container maxWidth="md">
        {children}
      </Container>
    </div>
  )
}
