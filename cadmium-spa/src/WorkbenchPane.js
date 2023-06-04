import './App.css'
import React, { useCallback, useEffect, useRef, useState } from 'react'
import { Canvas, useFrame } from '@react-three/fiber'
import { CameraControls } from '@react-three/drei'
import * as THREE from 'three'

import { useThree } from '@react-three/fiber'

function WorkbenchPane({ workbenchView }) {

  console.log("My workbenchview: ", workbenchView);
  let first_solid_mesh = null;
  if (workbenchView) {
    first_solid_mesh = workbenchView.solids[0].get("solid").get_mesh();
    // console.log(first_solid_mesh.get_mesh());
  }
  const mouseConfig = useCallback((camControls) => {
    if (camControls !== null) {
      camControls.mouseButtons.middle = 8
      camControls.mouseButtons.wheel = 8
      camControls.mouseButtons.right = 1
      camControls.mouseButtons.left = 0
    }
    //  NONE: 0,
    // 	ROTATE: 1,
    // 	TRUCK: 2,
    // 	OFFSET: 4,
    // 	DOLLY: 8,
    // see https://github.com/yomotsu/camera-controls/blob/29eac5b50e69f0cf6792b8c3c12f5c86ad621222/src/types.ts
  }, [])

  const overallScale = 1.1;

  return (
    <Canvas camera={{ fov: 45, position: [1 * overallScale, 2 * overallScale, 1 * overallScale], up: [0, 0, 1] }} style={{ height: 600 }}>
      <CameraControls ref={mouseConfig} />
      <ambientLight />
      <pointLight position={[10, 10, 10]} />

      <Solid mesh={first_solid_mesh}></Solid>
      <axesHelper></axesHelper>
    </Canvas>
  )
}

function Solid({ mesh }) {
  const ref = useRef()
  const [hovered, hover] = useState(false)
  const positions = new Float32Array(mesh.vertices.flatMap((v) => [v.x, v.y, v.z]));
  const normals = new Float32Array(mesh.normals.flatMap((v) => [v.x, v.y, v.z]));
  const indices = new Uint16Array(mesh.indices);

  console.log("as simple:", indices);
  return (
    <mesh
      ref={ref}
      onPointerOver={(event) => hover(false)}
      onPointerOut={(event) => hover(false)}
    >
      <bufferGeometry attach="geometry">
        <bufferAttribute
          attach="attributes-position"
          array={positions}
          count={positions.length / 3}
          itemSize={3}
        />
        <bufferAttribute
          attach="attributes-normal"
          array={normals}
          count={normals.length / 3}
          itemSize={3}
        />
        <bufferAttribute
          attach="index"
          array={indices}
          count={indices.length}
          itemSize={1}
        />
      </bufferGeometry>
      <meshStandardMaterial
        color={hovered ? 'hotpink' : 'blue'}
        side={THREE.DoubleSide}
      />
    </mesh>
  )
}

function Plane(props) {
  const ref = useRef()
  const [hovered, hover] = useState(false)
  return (
    <mesh
      {...props}
      ref={ref}
      onPointerOver={(event) => hover(false)}
      onPointerOut={(event) => hover(false)}
    >
      <planeGeometry args={[1, 1]} />
      <meshStandardMaterial
        color={hovered ? 'hotpink' : 'orange'}
        side={THREE.DoubleSide}
      />
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

export default WorkbenchPane
