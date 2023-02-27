import './App.css'
import React, { useRef, useState } from 'react'
import { Canvas, useFrame } from '@react-three/fiber'
import { CameraControls } from '@react-three/drei'
import * as THREE from 'three';

function MainViewport() {
  return (
    <Canvas style={{ height: 350 }}>
      <CameraControls />
      <ambientLight />
      <pointLight position={[10, 10, 10]} />
      <Plane></Plane>
    </Canvas>
  )
}

function Plane(props) {
  const ref = useRef()
  const [hovered, hover] = useState(false)
  return (
    <mesh
      {...props}
      ref={ref}
      onPointerOver={(event) => hover(true)}
      onPointerOut={(event) => hover(false)}
    >
      <planeGeometry args={[2, 2]} />
      <meshStandardMaterial color={hovered ? 'hotpink' : 'orange'} side={THREE.DoubleSide} />
    </mesh>
  )
}

function Box(props) {
  // This reference gives us direct access to the THREE.Mesh object
  const ref = useRef()
  // Hold state for hovered and clicked events
  const [hovered, hover] = useState(false)
  const [clicked, click] = useState(false)
  // Subscribe this component to the render-loop, rotate the mesh every frame
  useFrame((state, delta) => (ref.current.rotation.x += delta))
  // Return the view, these are regular Threejs elements expressed in JSX
  return (
    <mesh
      {...props}
      ref={ref}
      scale={clicked ? 1.5 : 1}
      onClick={(event) => click(!clicked)}
      onPointerOver={(event) => hover(true)}
      onPointerOut={(event) => hover(false)}
    >
      <boxGeometry args={[1, 1, 1]} />
      <meshStandardMaterial color={hovered ? 'hotpink' : 'orange'} />
    </mesh>
  )
}

export default MainViewport
