import useSWR from 'swr'
import { get_config, patch_config } from '@/cmds'
import { useLockFn } from 'ahooks'

export const useConfig = () => {
  const { data: config, mutate: mutateConfig } = useSWR('getConfig', get_config)

  const patchConfig = useLockFn(async (patch: Partial<Config>) => {
    await patch_config(patch)
    mutateConfig()
  })

  return {
    config,
    mutateConfig,
    patchConfig
  }
}
