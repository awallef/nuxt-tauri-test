import { invoke } from '@tauri-apps/api/tauri'

export const useLog = () => {
  
  const logDebugInfo = (message:string):void => {
    console.log(message);
    invoke('echo', { message: message})
  }

  return {logDebugInfo}
}
