import React from 'react'
import {makeStyles} from '@material-ui/core/styles'
import {Info} from '@material-ui/icons'
import {
  Button,
  Dialog,
  DialogActions,
  DialogContent,
  DialogContentText,
  DialogTitle,
  IconButton
} from '@material-ui/core'


const useStyles = makeStyles((theme) => ({
  menuButton: {
    marginLeft: theme.spacing(1)
  }
}))

export default function TopbarInfo() {
  const classes = useStyles()
  const [open, setOpen] = React.useState(false)

  const handleOpen = () => {
    setOpen(true)
  }

  const handleClose = () => {
    setOpen(false)
  }

  return (
    <div>
      <IconButton
        edge="start" color="inherit"
        aria-label="info" className={classes.menuButton}
        onClick={handleOpen}
      >
        <Info />
      </IconButton>

      <Dialog
        open={open}
        onClose={handleClose}
        aria-labelledby="alert-dialog-title"
        aria-describedby="alert-dialog-description">
        <DialogTitle id="alert-dialog-title">
          ✈️ AEROMIT BBTA3
        </DialogTitle>
        <DialogContent>
          <DialogContentText id="alert-dialog-description">
            Terima kasih telah menggunakan aplikasi Aeromit BBTA3. Apabila perlu informasi lebih lanjut,
            silahkan menghubungi kami.
          </DialogContentText>
        </DialogContent>
        <DialogActions>
          <Button href="https://discord.gg/Pnn6Htg" target="_blank" rel="noopener noreferrer"
                  color="primary"
                  autoFocus
          >
            Hubungi kami
          </Button>
        </DialogActions>
      </Dialog>
    </div>
  )
}
