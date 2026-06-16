import { invoke } from '@tauri-apps/api/core'
import { writeText, writeHtml } from '@tauri-apps/plugin-clipboard-manager'

export function useClipboard() {
  async function copyText(text: string): Promise<void> {
    await writeText(text)
  }

  /**
   * Copy text + images as rich content.
   * Writes plain text AND HTML (with embedded base64 images) to clipboard.
   * Compatible with WeChat, QQ, Word, and other rich text apps.
   */
  async function copyRich(text: string, imagePaths: string[]): Promise<void> {
    // Build HTML with embedded images
    const imageTags: string[] = []
    for (const filename of imagePaths) {
      const b64 = await invoke<string>('read_image_as_base64', { filename })
      const ext = filename.split('.').pop()?.toLowerCase() || 'png'
      const mime = ext === 'jpg' || ext === 'jpeg' ? 'image/jpeg' :
                   ext === 'gif' ? 'image/gif' :
                   ext === 'webp' ? 'image/webp' :
                   ext === 'bmp' ? 'image/bmp' : 'image/png'
      imageTags.push(`<img src="data:${mime};base64,${b64}" style="max-width:100%;margin:8px 0;display:block;" />`)
    }

    const escaped = text.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/\n/g, '<br>')
    const html = `<div>${escaped}</div>${imageTags.join('')}`

    // Write both: plain text fallback + HTML with images
    await writeHtml(html, text)
  }

  async function importImage(sourcePath: string): Promise<string> {
    return invoke<string>('import_image', { sourcePath })
  }

  async function getImageBase64(filename: string): Promise<string> {
    const b64 = await invoke<string>('read_image_as_base64', { filename })
    return `data:image/png;base64,${b64}`
  }

  async function deleteImage(filename: string): Promise<void> {
    await invoke('delete_image_file', { filename })
  }

  return {
    copyText,
    copyRich,
    importImage,
    getImageBase64,
    deleteImage,
  }
}
