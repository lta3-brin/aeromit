import React from "react"
import {useRouter} from "next/router"
import {IconButton, ListItemIcon, ListItemText, Menu, MenuItem} from "@material-ui/core";
import {MoreVert, AccountCircle, ExitToAppOutlined} from "@material-ui/icons";
import {makeStyles, withStyles} from "@material-ui/core/styles";


const useStyles = makeStyles((theme) => ({
  menuButton: {
    marginLeft: theme.spacing(1)
  }
}))

const StyledMenu = withStyles({
  paper: {
    border: '1px solid #d3d4d5',
  },
})((props) => (
  <Menu
    elevation={0}
    getContentAnchorEl={null}
    anchorOrigin={{
      vertical: 'bottom',
      horizontal: 'right',
    }}
    transformOrigin={{
      vertical: 'top',
      horizontal: 'right',
    }}
    {...props}
  />
));

export default function TopbarSetting() {
  const router = useRouter();
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

      <StyledMenu
        id="customized-menu"
        anchorEl={anchorEl}
        keepMounted
        open={Boolean(anchorEl)}
        onClose={handleClose}
      >
        <MenuItem onClick={() => {
          router.push("profil").then(() => {
            handleClose()
          })
        }}>
          <ListItemIcon>
            <AccountCircle fontSize="small" />
          </ListItemIcon>
          <ListItemText primary="Profil" />
        </MenuItem>

        <MenuItem onClick={() => {
          router.push("masuk").then(() => {
            handleClose()
          })
        }}>
          <ListItemIcon>
            <ExitToAppOutlined fontSize="small" />
          </ListItemIcon>
          <ListItemText primary="Keluar" />
        </MenuItem>
      </StyledMenu>
    </div>
  )
}
