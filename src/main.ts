import Danmaku from 'danmaku'
import { info } from 'tauri-plugin-log-api'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api'

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

const init = async () => {
  // 事件：弹幕发送。
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

  // 事件：显示弹幕。
  await listen('show', () => {
    app.style.marginTop = '0'
  })

  // 事件：隐藏弹幕。
  await listen('hide', () => {
    app.style.marginTop = '100%'
  })

  const reloadConfig = async () => {
    const config: {
      top_padding: number,
      bottom_padding: number,
      left_padding: number,
      right_padding: number,
    } = await invoke('get_config')

    await info(`加载配置：${JSON.stringify(config)}`)

    app.style.paddingTop = `${config.top_padding}px`
    app.style.paddingBottom = `${config.bottom_padding}px`
    app.style.paddingLeft = `${config.left_padding}px`
    app.style.paddingRight = `${config.right_padding}px`
  }

  // 事件：配置变更。
  await listen('config', reloadConfig)

  // 初始化配置。
  await reloadConfig()

  // 隐藏启动页面。
  setTimeout(() => (document.getElementById('splashscreen')!.style.opacity = '0'), 500)
  await info('弹幕姬已启动。')
};

init();