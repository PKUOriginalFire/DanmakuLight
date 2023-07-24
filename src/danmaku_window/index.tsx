import React, { useEffect } from 'react'
import Danmaku from 'danmaku'
import { info } from 'tauri-plugin-log-api'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api'
import './index.css'

/**
 * 事件：弹幕发送。
 */
interface DanmakuEvent {
  text: string
  size: number
  color: string
}

const App = () => {
  useEffect(() => {
    const app = document.getElementById('app')!
    const danmaku = new Danmaku({
      container: app,
      engine: 'dom'
    })
    danmaku.show()
    const reloadContainerConfig = async () => {
      const config: {
        top_padding: number
        bottom_padding: number
        left_padding: number
        right_padding: number
      } = await invoke('get_current_config')
      await info(`加载配置：${JSON.stringify(config)}`)
      app.style.paddingTop = `${config.top_padding}px`
      app.style.paddingBottom = `${config.bottom_padding}px`
      app.style.paddingLeft = `${config.left_padding}px`
      app.style.paddingRight = `${config.right_padding}px`
    }
    const setup = async () => {
      await listen<DanmakuEvent>('danmaku', async ({ payload }) => {
        const { text, size, color } = payload
        await info(`弹幕：${text}, size=${size}, color=${color}`)
        danmaku.emit({
          text,
          style: {
            fontSize: `${size}px`,
            fontWeight: 'bold',
            color,
            textShadow: '#000 1px 0px 1px, #000 0px 1px 1px, #000 0px -1px 1px, #000 -1px 0px 1px'
          }
        })
      })
      await listen('show', () => {
        app.style.marginTop = '0'
      })
      await listen('hide', () => {
        app.style.marginTop = '100%'
      })
      await listen('reload_config', reloadContainerConfig)
      await reloadContainerConfig()
      setTimeout(() => (document.getElementById('splashscreen')!.style.opacity = '0'), 500)
      await info('弹幕姬已启动。')
    }

    setup()
  })
  return (
    <div>
      <div id="app"></div>
      <div id="splashscreen">
        <p>Danmaku-Light</p>
      </div>
    </div>
  )
}

export default App
