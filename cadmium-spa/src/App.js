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

  const [colorScheme, setColorScheme] = useLocalStorage({
    key: 'mantine-color-scheme',
    defaultValue: 'light',
    getInitialValueInEffect: true,
  })

  const toggleColorScheme = (value) =>
    setColorScheme(value || (colorScheme === 'dark' ? 'light' : 'dark'))

  useHotkeys([['mod+J', () => toggleColorScheme()]])

  const runOnLoad = async () => {
    // console.log("Loading cadmium-js");
    await init();
    // console.log("Loaded cadmium-js");
    const project = new_project();
    console.log(project.workbench_names);
    const a = project.get_workbench("Workbench 1");
    console.log(a);
    console.log(a.get_steps());
    setProject(project);
  }

  useEffect(() => {
    runOnLoad();
  }, []);


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
            <MainWindow project={project}></MainWindow>
          </MantineProvider>
        </ColorSchemeProvider>
      </header>
    </div>
  )
}

export default App
