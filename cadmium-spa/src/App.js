import './App.css'
import React, { useEffect, useState } from 'react'
import { MantineProvider, ColorSchemeProvider } from '@mantine/core'
import { useHotkeys, useLocalStorage } from '@mantine/hooks'
import MainWindow from './MainWindow'


// import init { greet, demo } from "cadmium-js";
// import init, * as Truck from "./truck_js.js";
// import init, * as Truck from "truck-js";

import { default as init, new_project } from "cadmium-js";

function App() {
  const [project, setProject] = useState(null);
  const [, updateState] = React.useState();
  const forceUpdate = React.useCallback(() => updateState({}), []);


  const [colorScheme, setColorScheme] = useLocalStorage({
    key: 'mantine-color-scheme',
    defaultValue: 'light',
    getInitialValueInEffect: true,
  })

  const toggleColorScheme = (value) =>
    setColorScheme(value || (colorScheme === 'dark' ? 'light' : 'dark'))

  useHotkeys([['mod+J', () => toggleColorScheme()]])

  const runOnLoad = async () => {
    await init();
    const project = new_project();
    const a = project.get_workbench("Workbench 1");
    setProject(project);
  }

  useEffect(() => {
    runOnLoad();
  }, []);

  // const forceUpdate = () => {
  //   console.log("force update");
  //   setProject(project);
  // }



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
            <MainWindow project={project} forceUpdate={forceUpdate}></MainWindow>
          </MantineProvider>
        </ColorSchemeProvider>
      </header>
    </div >
  )
}

export default App
