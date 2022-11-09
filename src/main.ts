import Danmaku from 'danmaku'
import { info } from 'tauri-plugin-log-api'
import { listen } from '@tauri-apps/api/event'

/**
 * 事件：弹幕发送。
 */
interface DanmakuEvent {
  text: string
  size: number
  color: string
}

const app = document.getElementById('app')!
const danmaku = new Danmaku({
  container: app,
  engine: 'dom'
})
danmaku.show()

// 事件：弹幕发送。
listen<DanmakuEvent>('danmaku', ({ payload }) => {
  const { text, size, color } = payload
  info(`弹幕：${text}, size=${size}, color=${color}`)
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

// 事件：显示弹幕。
listen('show', () => {
  app.style.marginTop = '0'
})

// 事件：隐藏弹幕。
listen('hide', () => {
  app.style.marginTop = '100%'
})

// 隐藏启动页面。
setTimeout(() => (document.getElementById('splashscreen')!.style.opacity = '0'), 500)
info('弹幕姬已启动。')
