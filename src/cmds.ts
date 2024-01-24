import { invoke } from '@tauri-apps/api'

export async function get_config() {
  return invoke<Config | null>('get_current_config')
}

export async function patch_config(patch: Partial<Config>) {
  return invoke('patch_config', { patch })
}
