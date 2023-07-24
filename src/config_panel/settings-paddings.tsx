import React, { forwardRef, useImperativeHandle, useState } from 'react'
import { List, ListItem, ListItemText, TextField } from '@mui/material'
import { DialogRef, SubDialog } from './components/sub-dialog'
import { limitu16 } from '@/utils/limit-int'
import { useLockFn } from 'ahooks'
import { useConfig } from './hooks'

export const PaddingsDialog = forwardRef<DialogRef>((props, ref) => {
  const { config, patchConfig } = useConfig()

  const [open, setOpen] = useState(false)
  const [val, setVal] = useState({
    top_padding: config?.top_padding ?? 0,
    bottom_padding: config?.bottom_padding ?? 0,
    left_padding: config?.left_padding ?? 0,
    right_padding: config?.right_padding ?? 0
  })

  const onSave = useLockFn(async () => {
    try {
      await patchConfig(val)
      return setOpen(false)
    } catch {}
  })

  useImperativeHandle(ref, () => ({
    open: () => {
      setVal({
        top_padding: config?.top_padding ?? 0,
        bottom_padding: config?.bottom_padding ?? 0,
        left_padding: config?.left_padding ?? 0,
        right_padding: config?.right_padding ?? 0
      })
      setOpen(true)
    },
    close: () => {}
  }))

  return (
    <SubDialog
      open={open}
      title={'Danmaku Settings'}
      onClose={() => setOpen(false)}
      onCancel={() => setOpen(false)}
      onSave={onSave}
      contentSx={{ width: 300 }}
    >
      <List>
        <ListItem sx={{ padding: '5px 0px' }}>
          <ListItemText primary="Top padding" />
          <TextField
            size="small"
            autoComplete="off"
            sx={{ width: 100 }}
            value={val.top_padding}
            onChange={(e) => {
              setVal({ ...val, top_padding: limitu16('' + e.target.value) })
            }}
          />
        </ListItem>
        <ListItem sx={{ padding: '5px 0px' }}>
          <ListItemText primary="Left padding" />
          <TextField
            size="small"
            autoComplete="off"
            sx={{ width: 100 }}
            value={val.left_padding}
            onChange={(e) => {
              setVal({ ...val, left_padding: limitu16('' + e.target.value) })
            }}
          />
        </ListItem>
        <ListItem sx={{ padding: '5px 0px' }}>
          <ListItemText primary="Right padding" />
          <TextField
            size="small"
            autoComplete="off"
            sx={{ width: 100 }}
            value={val.right_padding}
            onChange={(e) => {
              setVal({ ...val, right_padding: limitu16('' + e.target.value) })
            }}
          />
        </ListItem>
        <ListItem sx={{ padding: '5px 0px' }}>
          <ListItemText primary="Bottom padding" />
          <TextField
            size="small"
            autoComplete="off"
            sx={{ width: 100 }}
            value={val.bottom_padding}
            onChange={(e) => {
              setVal({ ...val, bottom_padding: limitu16('' + e.target.value) })
            }}
          />
        </ListItem>
      </List>
    </SubDialog>
  )
})
