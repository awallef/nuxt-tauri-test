import { invoke } from '@tauri-apps/api/tauri'

export const useZip = () => {


  const zipFile = async (path:string, outputPath:string):Promise<any> => {
    return invoke('zip_file', { path: path, outputPath: outputPath})
  }

  const unzipFile = async (path:string, outputPath:string):Promise<any> => {
    return invoke('unzip_file', { path: path, outputPath: outputPath})
  }

  return {zipFile, unzipFile}
}
