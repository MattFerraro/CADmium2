import './App.css'
import React, { useRef, useState } from 'react'
import { AppShell, Navbar, Header } from '@mantine/core'
import MainViewport from './MainViewport'

function MainWindow() {
  return (
    <AppShell
      padding="md"
      navbar={
        <Navbar width={{ base: 300 }} height={540} p="xs">
          {/* Navbar content */}
        </Navbar>
      }
      header={
        <Header height={60} p="xs">
          {/* Header content */}
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
