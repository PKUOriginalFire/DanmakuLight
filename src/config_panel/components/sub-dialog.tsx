import React, { ReactNode } from 'react'
import {
  Dialog,
  DialogActions,
  Button,
  DialogTitle,
  DialogContent,
  SxProps,
  Theme
} from '@mui/material'

interface DialogProps {
  open: boolean
  title?: ReactNode
  children?: ReactNode
  onSave?: () => void
  onCancel?: () => void
  onClose?: () => void
  contentSx?: SxProps<Theme>
}

export interface DialogRef {
  open: () => void
}

export const SubDialog: React.FC<DialogProps> = (props) => {
  const { open, title, children, onSave, onCancel, onClose, contentSx } = props
  return (
    <Dialog open={open} onClose={onClose}>
      <DialogTitle>{title}</DialogTitle>
      <DialogContent sx={contentSx}>{children}</DialogContent>
      <DialogActions>
        <Button variant="outlined" onClick={onCancel}>
          {'Cancel'}
        </Button>
        <Button variant="contained" onClick={onSave}>
          {'Save'}
        </Button>
      </DialogActions>
    </Dialog>
  )
}
