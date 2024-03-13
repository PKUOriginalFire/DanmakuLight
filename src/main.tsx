import { createRoot } from 'react-dom/client'
import { BrowserRouter, Routes, Route } from 'react-router-dom'
import MainWindow from './danmaku_window'
import ConfigPanel from './config_panel'

const container = document.getElementById('root')

if (!container) {
  throw new Error(`No container 'root' found to render application`)
}

createRoot(container).render(
  <BrowserRouter>
    <Routes>
      <Route path="/" element={<MainWindow />}></Route>
      <Route path="/config_panel" element={<ConfigPanel />}></Route>
      <Route path="*" element={<MainWindow />}></Route>
    </Routes>
  </BrowserRouter>
)
