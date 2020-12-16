import React from 'react'
import color from '../styles/color'
import {ExitToApp, Info} from '@material-ui/icons'
import {makeStyles} from '@material-ui/core/styles'
import {AppBar, Box, IconButton, Toolbar, Typography} from '@material-ui/core'


const useStyles = makeStyles((theme) => ({
  root: {
    flexGrow: 1,
  },
  colorDefault: {
    backgroundColor: color.palette.bgnav.default
  },
  menuButton: {
    marginRight: theme.spacing(2),
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

          <Box ml={1}>
            <IconButton edge="start" color="inherit" aria-label="info">
              <Info />
            </IconButton>
          </Box>

          <Box ml={1}>
            <IconButton edge="start" color="inherit" aria-label="info">
              <ExitToApp />
            </IconButton>
          </Box>
        </Toolbar>
      </AppBar>
    </div>
  )
}
