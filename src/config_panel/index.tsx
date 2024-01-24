import { useEffect, useRef } from 'react'
import {
  IconButton,
  List,
  ListItem,
  ListItemText,
  Paper,
  ThemeProvider,
  Typography,
  createTheme
} from '@mui/material'
import { ErrorBoundary, FallbackProps } from 'react-error-boundary'
import { appWindow } from '@tauri-apps/api/window'
import { ArrowForward } from '@mui/icons-material'
import { DialogRef } from './components/sub-dialog'
import { SWRConfig } from 'swr'
import { WsDialog } from './settings-ws'
import { PaddingsDialog } from './settings-paddings'
import { WindowControl } from './window-control'
import './index.scss'

function Fallback({ error }: FallbackProps) {
  return (
    <div role="alert">
      <p>Error loading:</p>
      <p>{error.message}</p>
    </div>
  )
}

const theme = createTheme({
  palette: {
    primary: { main: '#d04c4c' },
    // secondary: { main: "#9c8790" },
    text: {
      primary: '#f38381'
      // secondary: "#909399"
    }
  }
})

const App = () => {
  useEffect(() => {
    window.addEventListener('keydown', (event) => {
      if (event.key === 'Escape') {
        appWindow.close()
      }
    })
  })
  const WsRef = useRef<DialogRef>(null)
  const PaddingsRef = useRef<DialogRef>(null)
  return (
    <SWRConfig>
      <ThemeProvider theme={theme}>
        <Paper
          className="layout"
          elevation={3}
          onPointerDown={(e: any) => {
            if (e.target?.dataset?.windrag) appWindow.startDragging()
          }}
          data-windrag
        >
          <ErrorBoundary FallbackComponent={Fallback} data-windrag>
            <div className='topbar'>
              <WindowControl />
            </div>
            
            <div className='content'>
              <div className='title' data-windrag>
                <Typography variant='h4' component='h1' data-windrag>
                  {'元火弹幕姬-配置面板'}
                </Typography>
              </div>
              
              <Paper elevation={3}>
                <List>
                  <WsDialog ref={WsRef} />
                  <PaddingsDialog ref={PaddingsRef} />
                  
                  <ListItem sx={{ padding: '5px 20px' }}>
                    <ListItemText primary="Websocket Settings" />
                    <IconButton color="inherit" size="small" onClick={() => WsRef.current?.open()}>
                      <ArrowForward />
                    </IconButton>
                  </ListItem>
                  
                  <ListItem sx={{ padding: '5px 20px' }}>
                    <ListItemText primary="Danmaku Settings" />
                    <IconButton
                      color="inherit"
                      size="small"
                      onClick={() => PaddingsRef.current?.open()}
                    >
                      <ArrowForward />
                    </IconButton>
                  </ListItem>
                </List>
              </Paper>
            </div>
          </ErrorBoundary>
        </Paper>
      </ThemeProvider>
    </SWRConfig>
  )
}

export default App;
