import React from "react"
import {IconButton, Menu, MenuItem} from "@material-ui/core";
import {MoreVert} from "@material-ui/icons";
import {makeStyles} from "@material-ui/core/styles";


const useStyles = makeStyles((theme) => ({
  menuButton: {
    marginLeft: theme.spacing(1)
  }
}))

export default function TopbarSetting() {
  const classes = useStyles()
  const [anchorEl, setAnchorEl] = React.useState(null)

  const handleClick = (event) => {
    setAnchorEl(event.currentTarget)
  }

  const handleClose = () => {
    setAnchorEl(null)
  }

  return (
    <div>
      <IconButton
        edge="start" color="inherit"
        aria-label="pengaturan" className={classes.menuButton}
        onClick={handleClick}
      >
        <MoreVert />
      </IconButton>

      <Menu
        id="simple-menu"
        anchorEl={anchorEl}
        keepMounted
        open={Boolean(anchorEl)}
        onClose={handleClose}
      >
        <MenuItem onClick={handleClose}>Profil</MenuItem>
        <MenuItem onClick={handleClose}>Keluar</MenuItem>
      </Menu>
    </div>
  )
}
