interface BotConfig {
  enable: boolean
  uin: number
  source_groups: Array<number>
}

interface Config {
  ws_port?: number
  top_padding?: number
  bottom_padding?: number
  left_padding?: number
  right_padding?: number
  bot_config?: BotConfig
}
