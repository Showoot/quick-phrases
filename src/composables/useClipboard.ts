import { invoke } from '@tauri-apps/api/core'
import { writeText } from '@tauri-apps/plugin-clipboard-manager'

/**
 * Composable for clipboard operations.
 * Text is copied via the clipboard plugin.
 * Images are resolved to base64 on the Rust side and then written.
 */
export function useClipboard() {
  /**
   * Copy text content to clipboard
   */
  async function copyText(text: string): Promise<void> {
    await writeText(text)
  }

  /**
   * Import an image from the filesystem into the app's storage.
   * Returns the stored filename.
   */
  async function importImage(sourcePath: string): Promise<string> {
    return invoke<string>('import_image', { sourcePath })
  }

  /**
   * Read a stored image as a base64 data URL
   */
  async function getImageBase64(filename: string): Promise<string> {
    const b64 = await invoke<string>('read_image_as_base64', { filename })
    return `data:image/png;base64,${b64}`
  }

  /**
   * Delete a stored image file
   */
  async function deleteImage(filename: string): Promise<void> {
    await invoke('delete_image_file', { filename })
  }

  return {
    copyText,
    importImage,
    getImageBase64,
    deleteImage,
  }
}
