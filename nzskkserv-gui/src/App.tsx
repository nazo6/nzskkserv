import { invoke } from '@tauri-apps/api/tauri'

export function App() {
  return <div>
    <div>
      <button onClick={() => {
        invoke("start_server")
      }}>start</button>
    </div>
  </div>
}
