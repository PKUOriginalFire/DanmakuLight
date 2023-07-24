import React, { forwardRef, useImperativeHandle, useState } from 'react'
import { List, ListItem, ListItemText, TextField, Switch } from '@mui/material'
import { DialogRef, SubDialog } from './components/sub-dialog'
import { limitu32 } from '@/utils/limit-int'
import { useLockFn } from 'ahooks'
import { useConfig } from './hooks'

export const BotDialog = forwardRef<DialogRef>((props, ref) => {
  const { config, patchConfig } = useConfig()

  const [open, setOpen] = useState(false)
  const [val, setVal] = useState({
    enable: false,
    uin: 0,
    source_groups: 0
  })

  const onSave = useLockFn(async () => {
    try {
      await patchConfig({
        bot_config: {
          enable: val?.enable,
          uin: val?.uin,
          source_groups: val?.source_groups ? [val?.source_groups] : []
        }
      })
      return setOpen(false)
    } catch {}
  })

  useImperativeHandle(ref, () => ({
    open: () => {
      setVal({
        enable: config?.bot_config?.enable ?? false,
        uin: config?.bot_config?.uin ?? 0,
        source_groups: config?.bot_config?.source_groups.length
          ? config?.bot_config?.source_groups[0]
          : 0
      })
      setOpen(true)
    },
    close: () => {}
  }))

  return (
    <SubDialog
      open={open}
      title={'QQBot Settings'}
      onClose={() => setOpen(false)}
      onCancel={() => setOpen(false)}
      onSave={onSave}
      contentSx={{ width: 300 }}
    >
      <List>
        <ListItem sx={{ padding: '5px 0px' }}>
          <ListItemText primary="enabled" />
          <Switch
            edge="end"
            checked={val.enable}
            onChange={(_, c) => {
              setVal((v) => ({ ...v, enable: c }))
            }}
          />
        </ListItem>
        <ListItem sx={{ padding: '5px 0px' }}>
          <ListItemText primary="uin" />
          <TextField
            size="small"
            autoComplete="off"
            sx={{ width: 140 }}
            value={val.uin}
            onChange={(e) => {
              setVal({ ...val, uin: limitu32('' + e.target.value) })
            }}
          />
        </ListItem>
        <ListItem sx={{ padding: '5px 0px' }}>
          <ListItemText primary="source group" />
          <TextField
            size="small"
            autoComplete="off"
            sx={{ width: 140 }}
            value={val.source_groups}
            onChange={(e) => {
              setVal({ ...val, source_groups: limitu32('' + e.target.value) })
            }}
          />
        </ListItem>
      </List>
    </SubDialog>
  )
})
