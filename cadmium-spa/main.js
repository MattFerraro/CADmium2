const { app, BrowserWindow } = require('electron')

const createWindow = () => {
  const win = new BrowserWindow({
    width: 1200,
    height: 800,
  })
  win.loadFile('build/index.html')
}

// On Windows or Linux, a user closing all windows means kill everything
app.on('window-all-closed', () => {
  // if (process.platform !== 'darwin') app.quit()
  app.quit()
})

app.whenReady().then(() => {
  createWindow()

  // on mac, clicking the icon in the tray only opens a new window if there is not one already open
  app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) createWindow()
  })
})
