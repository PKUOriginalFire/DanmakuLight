import { Button } from "@mui/material";
import { Close, CropSquare, Minimize } from "@mui/icons-material";
import { appWindow } from "@tauri-apps/api/window";


export const WindowControl = () => {
  
  return (
    <div>
      <Button
        size="small"
        onClick={() => appWindow.minimize()}
      >
        <Minimize fontSize="small" />
      </Button>
      <Button
        size="small"
        onClick={() => appWindow.toggleMaximize()}
      >
        <CropSquare fontSize="small" />
      </Button>
      <Button
        size="small"
        onClick={() => appWindow.close()}
      >
        <Close fontSize="small" />
      </Button>
    </div>
  )
}