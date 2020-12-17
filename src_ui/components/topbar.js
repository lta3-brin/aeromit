import React from 'react'
import color from '../styles/color'
import {makeStyles} from '@material-ui/core/styles'
import {ExitToApp, Info, Home} from '@material-ui/icons'
import {AppBar, IconButton, Toolbar, Tooltip, Typography, Zoom} from '@material-ui/core'


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
          <Typography variant="h6" className={classes.title}>
            AEROMIT
          </Typography>

          <Tooltip title="Utama" TransitionComponent={Zoom} arrow>
            <IconButton
              edge="start" color="inherit"
              aria-label="home" className={classes.menuButton}>
              <Home />
            </IconButton>
          </Tooltip>

          <Tooltip title="Informasi" TransitionComponent={Zoom} arrow>
            <IconButton
              edge="start" color="inherit"
              aria-label="info" className={classes.menuButton}>
              <Info />
            </IconButton>
          </Tooltip>

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
