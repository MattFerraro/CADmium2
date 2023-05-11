import './App.css'
import React, { useRef, useState } from 'react'
import { AppShell, Navbar, Header, Button } from '@mantine/core'
import MainViewport from './MainViewport'
import extrude_min from './extrude_min.svg';
import sketch_min from './sketch_min.svg';


function MainWindow() {
  return (
    <AppShell
      padding="md"
      navbar={
        <Navbar width={{ base: 300 }} height={540} p="xs">
          <div>
            History
          </div>
          <div>Solids</div>

        </Navbar>
      }
      header={
        <Header height={60} p="xs">
          <div>
            Actions:
            <Button variant='light' size='md'><img width={45} src={sketch_min}></img> New Sketch</Button>
            <Button variant='default' size='md'><img width={45} src={extrude_min}></img></Button>

          </div>
        </Header>
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
      <MainViewport></MainViewport>
    </AppShell>
  )
}

export default MainWindow
