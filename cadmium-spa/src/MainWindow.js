import './App.css'
import React, { useState } from 'react'
import { useDisclosure } from '@mantine/hooks';
import { NumberInput, Collapse, Menu, AppShell, Navbar, Header, Footer, Text, MediaQuery, Burger, useMantineTheme, Button, Tabs, ActionIcon } from '@mantine/core'
import WorkbenchPane from './WorkbenchPane'
import AssemblyPane from './AssemblyPane'
import extrude_min from './images/extrude_min.svg';
import sketch_min from './images/sketch_min.svg';
import point_min from './images/point_min.svg'
import plane_min from './images/plane_min.svg'
import cube_min from './images/cube_min.svg'
import logo from './logo.svg';
import { IconFileDownload, IconRulerMeasure } from '@tabler/icons-react';
import { NewPointStep, NewPlaneStep, NewSketchStep, NewExtrudeStep } from "cadmium-js";


// Cadmium blue:    #0a1195
// Cadmium red:     #e30022
// Cadmium yellow:  #fff600
// Cadmium green:   #006B3C

function MainWindow({ project, forceUpdate }) {
  const [opened, setOpened] = useState(false);
  const theme = useMantineTheme();
  const [activeTab, setActiveTab] = useState('Workbench 1');
  const [stepWithAttention, setStepWithAttention] = useState(null);
  const [mode, setMode] = useState("3D"); // can also be "sketch", changes to this affect which buttons
  // are shown on the top bar

  const workbench = project && project.get_workbench(activeTab);
  const steps = workbench && workbench.get_steps();
  let workbenchView = null;
  if (stepWithAttention !== null) {
    workbenchView = steps && workbench.create_view(stepWithAttention + 1);
  } else {
    workbenchView = steps && workbench.create_view(1000);
  }
  const solids = workbenchView && workbenchView.solids;
  console.log("solids:", solids);

  const setStepParameters = (step_name, parameter_names, parameter_values) => {
    console.log("SETTING PARAMS");
    project.set_step_parameters(activeTab, step_name, parameter_names, parameter_values);
    forceUpdate();
  };

  return (
    <AppShell
      padding="sm"
      navbarOffsetBreakpoint="sm"
      asideOffsetBreakpoint="sm"
      navbar={
        <Navbar p="md" hiddenBreakpoint="sm" hidden={!opened} width={{ sm: 200, lg: 300 }}>
          <Text>History</Text>
          {steps && steps.map((step, index) => {
            return <HistoryElement
              key={index}
              setMode={setMode}
              setStepParameters={setStepParameters}
              hasAttention={stepWithAttention === index}
              grabAttention={() => setStepWithAttention(index)}
              cedeAttention={() => setStepWithAttention(null)}
              step={step}>
            </HistoryElement>
          })}

          <hr style={{ width: "100%" }}></hr>

          {
            solids && solids.map((solid, index) => {

              let solid_name = solid.get("name")
              let solid_obj = solid.get("solid")

              let image = <img src={cube_min} width="30px"></img>;

              return <GeometryElement
                key={index}
                image={image}
                solid_name={solid_name}
                solid_obj={solid_obj}
              ></GeometryElement>
            })
          }

          {/* <Tabs value={activeTabLeft} onTabChange={setActiveTabLeft}>
            <Tabs.List>
              <Tabs.Tab value="Geometry">Geometry</Tabs.Tab>
              <Tabs.Tab value="Options">Options</Tabs.Tab>
            </Tabs.List>

            <Tabs.Panel value="Geometry">
              Geometry content
            </Tabs.Panel>
            <Tabs.Panel value="Options">Options content</Tabs.Panel>
          </Tabs> */}
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
                {mode === "3D" &&
                  <>
                    <ActionIcon size={'lg'} variant="subtle"><img src={sketch_min} width="30px"></img></ActionIcon>
                    <ActionIcon size={'lg'} variant="subtle"><img src={extrude_min} width="30px"></img></ActionIcon>
                  </>
                }

                {mode === "sketch" &&
                  <>
                    <ActionIcon size={'lg'} variant="default" >
                      <IconRulerMeasure size={16} />
                    </ActionIcon>
                  </>
                }

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

function download(content, mimeType, filename) {
  const a = document.createElement('a') // Create "a" element
  const blob = new Blob([content], { type: mimeType }) // Create a blob (file-like object)
  const url = URL.createObjectURL(blob) // Create an object URL from blob
  a.setAttribute('href', url) // Set "a" element link
  a.setAttribute('download', filename) // Set download filename
  a.click() // Start downloading
}

function GeometryElement({ image, solid_name, solid_obj }) {
  const onDownloadObj = () => {
    download(solid_obj.get_obj_text(), "text/plain", solid_name + ".obj");
  }

  const onDownloadStep = () => {
    download(solid_obj.get_step_text(), "text/plain", solid_name + ".step");
  }

  return <Menu withArrow>
    <Menu.Target>
      <div
        className='geometry-element'
        onContextMenu={(e) => {
          e.preventDefault(); // prevent the default behaviour when right clicked
          console.log("Right Click");
        }}>{image}<Text>{solid_name}</Text>
      </div>
    </Menu.Target>
    <Menu.Dropdown>
      <Menu.Label>Application</Menu.Label>
      <Menu.Item onClick={onDownloadObj} icon={<IconFileDownload size={16} />}>Download OBJ</Menu.Item>
      <Menu.Item onClick={onDownloadStep} icon={<IconFileDownload size={16} />}>Download STEP</Menu.Item>
    </Menu.Dropdown >
  </Menu >
}

function HistoryElement({ step, hasAttention, grabAttention, cedeAttention, setStepParameters, setMode }) {
  // const theme = useMantineTheme();

  let image = <img alt={"Nothing"} width="30px"></img>;
  if (step instanceof NewPointStep) {
    image = <img alt={"A Point"} src={point_min} width="30px"></img>
  }
  else if (step instanceof NewPlaneStep) {
    image = <img alt={"A Plane"} src={plane_min} width="30px"></img>
  }
  else if (step instanceof NewSketchStep) {
    image = <img alt={"A Sketch"} src={sketch_min} width="30px"></img>
  }
  else if (step instanceof NewExtrudeStep) {
    image = <img alt={"An Extrusion"} src={extrude_min} width="30px"></img>
  }

  function onDoubleClick(e) {
    e.preventDefault();
    console.log("doub click. Has attention? ", hasAttention);
    if (hasAttention) {
      cedeAttention();
    } else {
      if (step instanceof NewSketchStep) {
        setMode("sketch");
      }
      grabAttention();
    }
  }

  function onSave() {
    cedeAttention();
    setMode("3D");
  }

  let optionsForm = <div><Button onClick={onSave}>Save</Button></div>

  if (step instanceof NewExtrudeStep) {
    optionsForm = ExtrudeStepForm(step, setStepParameters, onSave);
  }

  if (step instanceof NewSketchStep) {
    optionsForm = SketchStepForm(step, setStepParameters, onSave);
  }

  // onClose={onCloseMenu}
  return <Menu withArrow opened={hasAttention}>
    <Menu.Target>
      <div>
        <div
          onDoubleClick={onDoubleClick}
          // onContextMenu={onRightClick}
          className='history-element' >
          {image}
          <Text>{step.name}</Text>

        </div>
        <Collapse in={hasAttention}>
          {optionsForm}
        </Collapse>
      </div>
    </Menu.Target>
  </Menu >
}

const SketchStepForm = (step, setStepParameters, close) => {
  return <div className='options-form'>
    <div className='options-form-element'>
    </div>
    <div className='options-form-element'>
      <Button size='sm' onClick={close}>
        Save
      </Button>
    </div>
  </div>
}


const ExtrudeStepForm = (step, setStepParameters, close) => {
  const [depthValue, setDepthValue] = useState(20);

  function onSave(e) {
    // console.log("save:", depthValue, step.name, activeTab);
    setStepParameters(step.name, ["depth"], [depthValue]);
    close();
  }

  return <div className='options-form'>
    <div className='options-form-element'>
      <NumberInput
        defaultValue={depthValue}
        placeholder="Depth"
        label="Depth"
        precision={3}
        size="md"
        value={depthValue}
        onChange={setDepthValue}
        hideControls
      />
    </div>
    <div className='options-form-element'>
      <Button size='sm' onClick={() => onSave(step.name)}>
        Save
      </Button>
    </div>
  </div>
}

export default MainWindow
