import './App.css'
import React, { useEffect, useRef, useState } from 'react'
import { AppShell, Navbar, Header } from '@mantine/core'
import { MantineProvider, ColorSchemeProvider } from '@mantine/core'
import { useHotkeys, useLocalStorage } from '@mantine/hooks'
import MainWindow from './MainWindow'
// import init, { greet, demo } from "hello-wasm";
// import init, * as Truck from "./truck_js.js";
// import init, * as Truck from "truck-js";

function App() {
  const [colorScheme, setColorScheme] = useLocalStorage({
    key: 'mantine-color-scheme',
    defaultValue: 'light',
    getInitialValueInEffect: true,
  })

  const toggleColorScheme = (value) =>
    setColorScheme(value || (colorScheme === 'dark' ? 'light' : 'dark'))

  useHotkeys([['mod+J', () => toggleColorScheme()]])

  return (
    <div className="App">
      <header className="App-header">
        <ColorSchemeProvider
          colorScheme={colorScheme}
          toggleColorScheme={toggleColorScheme}
        >
          <MantineProvider
            theme={{ colorScheme }}
            withGlobalStyles
            withNormalizeCSS
          >
            <MainWindow></MainWindow>
          </MantineProvider>
        </ColorSchemeProvider>
      </header>
    </div>
  )
}

export default App
