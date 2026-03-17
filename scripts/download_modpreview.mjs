import { createWriteStream, mkdirSync } from 'fs'
import { pipeline } from 'stream/promises'
import { get } from 'https'

const URL = 'https://github.com/bruhnn/bd2modpreview/releases/latest/download/BD2ModPreview.exe'
const OUT = 'src-tauri/resources/BD2ModPreview.exe'

mkdirSync('src-tauri/resources', { recursive: true })

get(URL, { headers: { 'User-Agent': 'BD2ModManager' } }, async res => {
  if (res.statusCode === 302 || res.statusCode === 301) {
    get(res.headers.location, async res2 => {
      await pipeline(res2, createWriteStream(OUT))
      console.log('Downloaded BD2ModPreview.exe')
    })
  }
})