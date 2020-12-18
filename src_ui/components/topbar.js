import React from "react"
import color from "../styles/color"
import {makeStyles} from "@material-ui/core/styles"
import {ExitToApp, Home} from "@material-ui/icons"
import {AppBar, IconButton, Toolbar, Tooltip, Typography, Zoom} from "@material-ui/core"
import InfoComponent from "./modals/info"


const useStyles = makeStyles((theme) => ({
  root: {
    flexGrow: 1,
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
    <div className={classes.root}>
      <AppBar position="static" className={classes.colorDefault}>
        <Toolbar>
          <Typography variant="h5" className={classes.title}>
            AEROMIT
          </Typography>

          <Tooltip title="Utama" TransitionComponent={Zoom} arrow>
            <IconButton
              edge="start" color="inherit"
              aria-label="home" className={classes.menuButton}>
              <Home />
            </IconButton>
          </Tooltip>

          <InfoComponent />

          <Tooltip title="Keluar" TransitionComponent={Zoom} arrow>
            <IconButton
              edge="start" color="inherit"
              aria-label="exit" className={classes.menuButton}>
              <ExitToApp />
            </IconButton>
          </Tooltip>
        </Toolbar>
      </AppBar>
    </div>
  )
}
