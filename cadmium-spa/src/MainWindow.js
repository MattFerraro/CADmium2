import './App.css'
import React, { useRef, useState } from 'react'
import { AppShell, Navbar, Header, Footer, Text, MediaQuery, Burger, useMantineTheme, Button, Tabs, ActionIcon } from '@mantine/core'
// import MainViewport from './MainViewport'
import WorkbenchPane from './WorkbenchPane'
import AssemblyPane from './AssemblyPane'
import extrude_min from './images/extrude_min.svg';
import sketch_min from './images/sketch_min.svg';
import point_min from './images/point_min.svg'
import plane_min from './images/plane_min.svg'
import { act } from '@react-three/fiber';
import logo from './logo.svg';
// import { IconSettings } from '@tabler/icons-react';
import { NewPointStep, NewPlaneStep, NewSketchStep, NewExtrudeStep } from "cadmium-js";


// Cadmium blue:    #0a1195
// Cadmium red:     #e30022
// Cadmium yellow:  #fff600
// Cadmium green:   #006B3C

function MainWindow({ project }) {
  const [opened, setOpened] = useState(false);
  const theme = useMantineTheme();
  const [activeTab, setActiveTab] = useState('Workbench 1');


  const workbench = project && project.get_workbench(activeTab);
  const steps = workbench && workbench.get_steps();
  const workbenchView = steps && workbench.create_view(1000);

  return (
    <AppShell
      padding="sm"
      navbarOffsetBreakpoint="sm"
      asideOffsetBreakpoint="sm"
      navbar={
        <Navbar p="md" hiddenBreakpoint="sm" hidden={!opened} width={{ sm: 200, lg: 300 }}>
          <Text>History</Text>
          {steps && steps.map((step, index) => {
            let image = null;
            if (step instanceof NewPointStep) {
              image = <img src={point_min} width="30px"></img>
            }
            else if (step instanceof NewPlaneStep) {
              image = <img src={plane_min} width="30px"></img>
            }
            else if (step instanceof NewSketchStep) {
              image = <img src={sketch_min} width="30px"></img>
            }
            else if (step instanceof NewExtrudeStep) {
              image = <img src={extrude_min} width="30px"></img>
            }

            return <div className='history-element' key={index}>{image}<Text>{step.name}</Text></div>
          })}

          <hr style={{ width: "100%" }}></hr>
          <Text>Solids</Text>
        </ Navbar>
      }
      // TODO: revive and put stuff here? if required.
      // aside={
      //   <MediaQuery smallerThan="sm" styles={{ display: 'none' }}>
      //     <Aside p="md" hiddenBreakpoint="sm" width={{ sm: 200, lg: 300 }}>
      //       <Text>Application sidebar</Text>
      //     </Aside>
      //   </MediaQuery>
      // }
      footer={
        < Footer height={60} p="md" style={{
          height: "40px", paddingTop: "0px", paddingBottom: "0px",
        }}>
          < Tabs value={activeTab} onTabChange={setActiveTab} inverted variant="outline" >
            <Tabs.List>
              <Tabs.Tab value="Workbench 1">Workbench 1</Tabs.Tab>
              <Tabs.Tab value="Assembly 1">Assembly 1</Tabs.Tab>
            </Tabs.List>
          </Tabs >
        </Footer >
      }
      header={
        < Header height={{ base: 50, md: 70 }} p="md" >
          <div style={{ display: 'flex', alignItems: 'center', height: '100%' }}>
            <MediaQuery largerThan="sm" styles={{ display: 'none' }}>
              <Burger
                opened={opened}
                onClick={() => setOpened((o) => !o)}
                size="sm"
                color={theme.colors.gray[6]}
                mr="xl"
              />
            </MediaQuery>

            <div className='header-row'>
              <div className='logo-container'>
                <img src={logo} width={40}></img>
                <Text fz="xl" fw={700}>CADmium</Text>
              </div>
              <Text fz="xl">{project && project.name}</Text>
              <div className='actions-container'>
                <ActionIcon size={'lg'} variant="subtle"><img src={sketch_min} width="30px"></img></ActionIcon>
                <ActionIcon size={'lg'} variant="subtle"><img src={extrude_min} width="30px"></img></ActionIcon>
              </div>
            </div>

          </div>
        </Header >
      }
      styles={(theme) => ({
        main: {
          backgroundColor:
            theme.colorScheme === 'dark'
              ? theme.colors.dark[8]
              : theme.colors.gray[0],
        },
      })}
    >

      {activeTab === "Workbench 1" && <WorkbenchPane workbenchView={workbenchView}></WorkbenchPane>}
      {activeTab === "Assembly 1" && <AssemblyPane></AssemblyPane>}

    </AppShell >
  )
}

export default MainWindow
