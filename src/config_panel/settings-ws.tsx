import { forwardRef, useImperativeHandle, useState } from 'react'
import { List, ListItem, ListItemText, TextField } from '@mui/material'
import { DialogRef, SubDialog } from './components/sub-dialog'
import { limitu16 } from '@/utils/limit-int'
import { useLockFn } from 'ahooks'
import { useConfig } from './hooks'

export const WsDialog = forwardRef<DialogRef>((_props, ref) => {
  const { config, patchConfig } = useConfig()

  const [open, setOpen] = useState(false)
  const [val, setVal] = useState(config?.ws_port ?? 3210)

  const onSave = useLockFn(async () => {
    if (val === config?.ws_port) return setOpen(false)
    await patchConfig({ ws_port: val })
    return setOpen(false)
  })

  useImperativeHandle(ref, () => ({
    open: () => {
      setVal(config?.ws_port ?? 3210)
      setOpen(true)
    },
    close: () => {}
  }))

  return (
    <SubDialog
      open={open}
      title={'Websocket Settings'}
      onClose={() => setOpen(false)}
      onCancel={() => setOpen(false)}
      onSave={onSave}
      contentSx={{ width: 300 }}
    >
      <List>
        <ListItem sx={{ padding: '5px 0px' }}>
          <ListItemText primary="Websocket Port" />
          <TextField
            size="small"
            autoComplete="off"
            sx={{ width: 100 }}
            value={val}
            onChange={(e) => {
              setVal(limitu16('' + e.target.value))
            }}
          />
        </ListItem>
      </List>
    </SubDialog>
  )
})
