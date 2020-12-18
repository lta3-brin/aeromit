import React from "react"
import Link from "next/link"
import color from "../styles/color"
import {makeStyles} from "@material-ui/core/styles"
import {Home} from "@material-ui/icons"
import {AppBar, Container, IconButton, Toolbar, Typography} from "@material-ui/core"
import TopbarInfo from "./modals/topbar_info"
import TopbarSettings from "./menu/topbar_settings"


const useStyles = makeStyles((theme) => ({
  root: {
    padding: "0",
  },
  colorDefault: {
    backgroundColor: color.palette.bgnav.default
  },
  menuButton: {
    marginLeft: theme.spacing(1),
  },
  title: {
    flexGrow: 1,
    fontWeight: "bold"
  },
}))

export default function Topbar() {
  const classes = useStyles()

  return (
    <AppBar position="static" className={classes.colorDefault}>
      <Container maxWidth="md">
        <Toolbar className={classes.root}>
          <Typography variant="h5" className={classes.title}>
            AEROMIT
          </Typography>

          <Link href="/" passHref>
            <IconButton
              edge="start" color="inherit"
              aria-label="home" className={classes.menuButton}>
              <Home />
            </IconButton>
          </Link>

          <TopbarInfo />

          <TopbarSettings />
        </Toolbar>
      </Container>
    </AppBar>
  )
}
